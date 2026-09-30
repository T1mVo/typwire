//! Converts directly between an arbitrary `T: Serialize`/`Deserialize` and a
//! [`Value`] tree, with no intermediate bytes at all.
//!
//! Replaces what `wire.rs`'s `value_from_owned`/`value_to_owned` used to do
//! by round-tripping through real CBOR bytes (`T` -> CBOR bytes -> `Value`,
//! and the reverse) -- `cbor4ii` doesn't expose a byte-free "serialize into
//! `Value`"/"deserialize from `Value`" API (checked: `cbor4ii::serde` only
//! has `to_vec`/`from_slice`/`to_writer`/`from_reader`, all byte-level), so
//! this crate writes its own, the same "custom (de)serializer wrapping a
//! generic tree type" idiom `LenientDeserializer` (`format/cbor.rs`) already
//! uses for a different problem. `Json` is the only caller (`format/mod.rs`);
//! `Cbor` never needs this, since it serializes `T` directly in one real
//! pass.
//!
//! Every wire-shape decision documented for the byte-round-trip version
//! this replaces is preserved exactly -- confirmed empirically by capturing
//! the old version's output for a representative set of values (`()`,
//! `None`/`Some`, `Bytes`, negative/`i64::MAX` integers, a native type, an
//! `Any`, a `OneOf2`) and diffing every byte against this version's output
//! before it replaced the old one. In particular: `()` still becomes an
//! *empty array*, not `null` -- `cbor4ii`'s own serializer encodes a Rust
//! unit that way (confirmed by inspecting its `Value::Array(vec![])`
//! output), and nothing here treats `Bytes`/`none` specially either -- by
//! the time `wire.rs::value_to_owned` calls into this module,
//! `rewrite_array_to_bytes`/the CBOR-native path has already turned any
//! tagged-array/tagged-`none` dictionary back into a real `Value::Bytes`/
//! `Value::Null`, so decoding here is just an ordinary self-describing
//! `Value` -> `T` walk with no tag awareness needed at all.

use std::fmt;

use cbor4ii::core::Value;
use serde::de::{
    DeserializeOwned, DeserializeSeed, Deserializer, EnumAccess, MapAccess, SeqAccess, VariantAccess,
    Visitor,
};
use serde::ser::{
    Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant, Serializer,
};

#[derive(Debug)]
pub(crate) struct ValueError(String);

impl ValueError {
    // A plain inherent constructor, not just the trait methods below: this
    // type implements *both* `serde::ser::Error` and `serde::de::Error`
    // (both define a `custom` associated function), so `ValueError::custom`
    // alone is ambiguous at every call site in this file -- this inherent
    // one is what they actually call.
    fn custom<T: fmt::Display>(message: T) -> Self {
        Self(message.to_string())
    }
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ValueError {}

impl serde::ser::Error for ValueError {
    fn custom<T: fmt::Display>(message: T) -> Self {
        Self::custom(message)
    }
}

impl serde::de::Error for ValueError {
    fn custom<T: fmt::Display>(message: T) -> Self {
        Self::custom(message)
    }
}

/// Build a [`Value`] tree directly from `value`, with no CBOR bytes in
/// between.
///
/// Only `Json::encode` calls this (`from_value` below, unlike this
/// function, is also needed in a CBOR-only build -- see this crate's
/// `format/mod.rs`), so a build without the `json` feature never calls it;
/// `allow(dead_code)` covers exactly that build.
#[cfg_attr(not(feature = "json"), allow(dead_code))]
pub(crate) fn to_value<T: Serialize + ?Sized>(value: &T) -> Result<Value, ValueError> {
    value.serialize(ValueSerializer)
}

/// The mirror of [`to_value`]: reconstruct `T` directly from a [`Value`]
/// tree, with no CBOR bytes in between.
pub(crate) fn from_value<T: DeserializeOwned>(value: Value) -> Result<T, ValueError> {
    T::deserialize(ValueDeserializer(value))
}

// --- Serialize: T -> Value -------------------------------------------------

struct ValueSerializer;

/// Shared by `SerializeSeq`/`SerializeTuple`/`SerializeTupleStruct` (never
/// tagged) and `SerializeTupleVariant` (tagged with the variant name on
/// `end`) -- one collector type for every "sequence of values" shape.
struct ValueSeqSerializer {
    items: Vec<Value>,
    variant: Option<&'static str>,
}

/// Shared by `SerializeMap`/`SerializeStruct` (never tagged) and
/// `SerializeStructVariant` (tagged on `end`) -- one collector type for
/// every "sequence of key/value pairs" shape.
struct ValueMapSerializer {
    entries: Vec<(Value, Value)>,
    next_key: Option<Value>,
    variant: Option<&'static str>,
}

impl Serializer for ValueSerializer {
    type Ok = Value;
    type Error = ValueError;
    type SerializeSeq = ValueSeqSerializer;
    type SerializeTuple = ValueSeqSerializer;
    type SerializeTupleStruct = ValueSeqSerializer;
    type SerializeTupleVariant = ValueSeqSerializer;
    type SerializeMap = ValueMapSerializer;
    type SerializeStruct = ValueMapSerializer;
    type SerializeStructVariant = ValueMapSerializer;

    fn serialize_bool(self, v: bool) -> Result<Value, ValueError> {
        Ok(Value::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Value, ValueError> {
        Ok(Value::Integer(v.into()))
    }

    fn serialize_i16(self, v: i16) -> Result<Value, ValueError> {
        Ok(Value::Integer(v.into()))
    }

    fn serialize_i32(self, v: i32) -> Result<Value, ValueError> {
        Ok(Value::Integer(v.into()))
    }

    fn serialize_i64(self, v: i64) -> Result<Value, ValueError> {
        Ok(Value::Integer(v.into()))
    }

    fn serialize_i128(self, v: i128) -> Result<Value, ValueError> {
        Ok(Value::Integer(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Value, ValueError> {
        Ok(Value::Integer(v.into()))
    }

    fn serialize_u16(self, v: u16) -> Result<Value, ValueError> {
        Ok(Value::Integer(v.into()))
    }

    fn serialize_u32(self, v: u32) -> Result<Value, ValueError> {
        Ok(Value::Integer(v.into()))
    }

    fn serialize_u64(self, v: u64) -> Result<Value, ValueError> {
        Ok(Value::Integer(v.into()))
    }

    fn serialize_u128(self, v: u128) -> Result<Value, ValueError> {
        i128::try_from(v)
            .map(Value::Integer)
            .map_err(|_| ValueError::custom(format!("u128 value {v} out of range for i128")))
    }

    fn serialize_f32(self, v: f32) -> Result<Value, ValueError> {
        Ok(Value::Float(v.into()))
    }

    fn serialize_f64(self, v: f64) -> Result<Value, ValueError> {
        Ok(Value::Float(v))
    }

    fn serialize_char(self, v: char) -> Result<Value, ValueError> {
        Ok(Value::Text(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Value, ValueError> {
        Ok(Value::Text(v.to_owned()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Value, ValueError> {
        Ok(Value::Bytes(v.to_owned()))
    }

    fn serialize_none(self) -> Result<Value, ValueError> {
        Ok(Value::Null)
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Value, ValueError> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Value, ValueError> {
        // Matches `cbor4ii`'s own wire shape for a Rust unit, which is an
        // empty array, not a null -- see this module's own doc comment.
        Ok(Value::Array(Vec::new()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value, ValueError> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
    ) -> Result<Value, ValueError> {
        Ok(Value::Text(variant.to_owned()))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Value, ValueError> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value, ValueError> {
        Ok(Value::Map(vec![(
            Value::Text(variant.to_owned()),
            value.serialize(ValueSerializer)?,
        )]))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<ValueSeqSerializer, ValueError> {
        Ok(ValueSeqSerializer {
            items: Vec::with_capacity(len.unwrap_or(0)),
            variant: None,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<ValueSeqSerializer, ValueError> {
        Ok(ValueSeqSerializer {
            items: Vec::with_capacity(len),
            variant: None,
        })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<ValueSeqSerializer, ValueError> {
        Ok(ValueSeqSerializer {
            items: Vec::with_capacity(len),
            variant: None,
        })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<ValueSeqSerializer, ValueError> {
        Ok(ValueSeqSerializer {
            items: Vec::with_capacity(len),
            variant: Some(variant),
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<ValueMapSerializer, ValueError> {
        Ok(ValueMapSerializer {
            entries: Vec::with_capacity(len.unwrap_or(0)),
            next_key: None,
            variant: None,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<ValueMapSerializer, ValueError> {
        Ok(ValueMapSerializer {
            entries: Vec::with_capacity(len),
            next_key: None,
            variant: None,
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<ValueMapSerializer, ValueError> {
        Ok(ValueMapSerializer {
            entries: Vec::with_capacity(len),
            next_key: None,
            variant: Some(variant),
        })
    }

    // `Value` is a plain in-memory tree, not text -- matches the CBOR path,
    // which is binary too. Nothing in this crate's types branches on this
    // (every native type's wire shape comes from an explicit shadow struct
    // with plain fields, never a `#[serde(...)]` attribute conditioned on
    // it), so the choice is inert; picked for consistency with `Cbor`
    // rather than for any observed behavioral difference.
    fn is_human_readable(&self) -> bool {
        false
    }
}

impl SerializeSeq for ValueSeqSerializer {
    type Ok = Value;
    type Error = ValueError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), ValueError> {
        self.items.push(value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value, ValueError> {
        Ok(Value::Array(self.items))
    }
}

impl SerializeTuple for ValueSeqSerializer {
    type Ok = Value;
    type Error = ValueError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), ValueError> {
        self.items.push(value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value, ValueError> {
        Ok(Value::Array(self.items))
    }
}

impl SerializeTupleStruct for ValueSeqSerializer {
    type Ok = Value;
    type Error = ValueError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), ValueError> {
        self.items.push(value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value, ValueError> {
        Ok(Value::Array(self.items))
    }
}

impl SerializeTupleVariant for ValueSeqSerializer {
    type Ok = Value;
    type Error = ValueError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), ValueError> {
        self.items.push(value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value, ValueError> {
        let variant = self
            .variant
            .expect("SerializeTupleVariant always has a variant name");
        Ok(Value::Map(vec![(
            Value::Text(variant.to_owned()),
            Value::Array(self.items),
        )]))
    }
}

impl SerializeMap for ValueMapSerializer {
    type Ok = Value;
    type Error = ValueError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), ValueError> {
        self.next_key = Some(key.serialize(ValueSerializer)?);
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), ValueError> {
        let key = self
            .next_key
            .take()
            .ok_or_else(|| ValueError::custom("serialize_value called before serialize_key"))?;
        self.entries.push((key, value.serialize(ValueSerializer)?));
        Ok(())
    }

    fn end(self) -> Result<Value, ValueError> {
        Ok(Value::Map(self.entries))
    }
}

impl SerializeStruct for ValueMapSerializer {
    type Ok = Value;
    type Error = ValueError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), ValueError> {
        self.entries
            .push((Value::Text(key.to_owned()), value.serialize(ValueSerializer)?));
        Ok(())
    }

    fn end(self) -> Result<Value, ValueError> {
        Ok(Value::Map(self.entries))
    }
}

impl SerializeStructVariant for ValueMapSerializer {
    type Ok = Value;
    type Error = ValueError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), ValueError> {
        self.entries
            .push((Value::Text(key.to_owned()), value.serialize(ValueSerializer)?));
        Ok(())
    }

    fn end(self) -> Result<Value, ValueError> {
        let variant = self
            .variant
            .expect("SerializeStructVariant always has a variant name");
        Ok(Value::Map(vec![(
            Value::Text(variant.to_owned()),
            Value::Map(self.entries),
        )]))
    }
}

// --- Deserialize: Value -> T -------------------------------------------------

/// A local wrapper around [`Value`] -- `Deserializer` (like every serde
/// trait) can only be implemented for a type defined in this crate, and
/// both `Value` and `Deserializer` are foreign (Rust's orphan rule; hit
/// this the first time this file compiled). Every other type in this half
/// of the module wraps `Value` in this the same way, for the same reason.
struct ValueDeserializer(Value);

/// One macro body for every fixed-width integer type: `Value::Integer`
/// holds `i128`, the widest integer any of these can come from, so each
/// narrower target just needs its own checked `TryFrom` (matching the CBOR
/// path's own typed decode, which is checked the same way -- see
/// `wire.rs`'s module doc comment on `LenientDeserializer`) and its own
/// `Visitor::visit_*` call. Anything that isn't a `Value::Integer` at all
/// falls back to `deserialize_any`, the same "let the value describe
/// itself" behavior every other typed method here uses.
macro_rules! deserialize_int {
    ($method:ident, $visit:ident, $ty:ty) => {
        fn $method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
            match self.0 {
                Value::Integer(v) => match <$ty>::try_from(v) {
                    Ok(v) => visitor.$visit(v),
                    Err(_) => Err(ValueError::custom(format!(
                        "integer {v} out of range for {}",
                        stringify!($ty)
                    ))),
                },
                other => ValueDeserializer(other).deserialize_any(visitor),
            }
        }
    };
}

impl<'de> Deserializer<'de> for ValueDeserializer {
    type Error = ValueError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
        match self.0 {
            Value::Null => visitor.visit_unit(),
            Value::Bool(v) => visitor.visit_bool(v),
            Value::Integer(v) => match i64::try_from(v) {
                Ok(v) => visitor.visit_i64(v),
                Err(_) => match u64::try_from(v) {
                    Ok(v) => visitor.visit_u64(v),
                    Err(_) => visitor.visit_i128(v),
                },
            },
            Value::Float(v) => visitor.visit_f64(v),
            Value::Bytes(v) => visitor.visit_byte_buf(v),
            Value::Text(v) => visitor.visit_string(v),
            Value::Array(items) => visitor.visit_seq(ValueSeqAccess {
                iter: items.into_iter(),
            }),
            Value::Map(entries) => visitor.visit_map(ValueMapAccess {
                iter: entries.into_iter(),
                value: None,
            }),
            // A tag never legitimately reaches this module -- decode's tag
            // dispatch (`decode-tagged`) already turns every
            // `typwire-type`-carrying dictionary into either a plain
            // `Value::Map` (for real native types, handled recursively
            // above `value_to_owned`'s call site) or a `Value::Bytes`/
            // `Value::Null` (via the rewrite functions, before this stage
            // is ever reached). Unwrap rather than error, on the chance a
            // raw CBOR tag byte somehow survived this far -- consistent
            // with `Cbor`'s own decode, which also treats an unexpected
            // tag as something to route around rather than a hard failure
            // this deep in the pipeline.
            Value::Tag(_, inner) => ValueDeserializer(*inner).deserialize_any(visitor),
            // `Value` is `#[non_exhaustive]` (new CBOR major types could be
            // added upstream); every variant that exists today is handled
            // above.
            other => Err(ValueError::custom(format!(
                "unsupported value variant: {other:?}"
            ))),
        }
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
        match self.0 {
            Value::Bool(v) => visitor.visit_bool(v),
            other => ValueDeserializer(other).deserialize_any(visitor),
        }
    }

    deserialize_int!(deserialize_i8, visit_i8, i8);
    deserialize_int!(deserialize_i16, visit_i16, i16);
    deserialize_int!(deserialize_i32, visit_i32, i32);
    deserialize_int!(deserialize_i64, visit_i64, i64);
    deserialize_int!(deserialize_i128, visit_i128, i128);
    deserialize_int!(deserialize_u8, visit_u8, u8);
    deserialize_int!(deserialize_u16, visit_u16, u16);
    deserialize_int!(deserialize_u32, visit_u32, u32);
    deserialize_int!(deserialize_u64, visit_u64, u64);
    deserialize_int!(deserialize_u128, visit_u128, u128);

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
        match self.0 {
            Value::Float(v) => visitor.visit_f32(v as f32),
            other => ValueDeserializer(other).deserialize_any(visitor),
        }
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
        match self.0 {
            Value::Float(v) => visitor.visit_f64(v),
            other => ValueDeserializer(other).deserialize_any(visitor),
        }
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
        match self.0 {
            Value::Text(v) => visitor.visit_string(v),
            other => ValueDeserializer(other).deserialize_any(visitor),
        }
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
        match self.0 {
            Value::Text(v) => visitor.visit_string(v),
            other => ValueDeserializer(other).deserialize_any(visitor),
        }
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
        match self.0 {
            Value::Bytes(v) => visitor.visit_byte_buf(v),
            other => ValueDeserializer(other).deserialize_any(visitor),
        }
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
        match self.0 {
            Value::Null => visitor.visit_none(),
            other => visitor.visit_some(ValueDeserializer(other)),
        }
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
        // Matches `cbor4ii`'s own wire shape for a Rust unit -- an empty
        // array, not a null -- see this module's own doc comment.
        match self.0 {
            Value::Array(items) if items.is_empty() => visitor.visit_unit(),
            other => Err(ValueError::custom(format!(
                "expected unit (an empty array), found {other:?}"
            ))),
        }
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, ValueError> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, ValueError> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
        match self.0 {
            Value::Array(items) => visitor.visit_seq(ValueSeqAccess {
                iter: items.into_iter(),
            }),
            other => ValueDeserializer(other).deserialize_any(visitor),
        }
    }

    fn deserialize_tuple<V: Visitor<'de>>(
        self,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, ValueError> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, ValueError> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
        match self.0 {
            Value::Map(entries) => visitor.visit_map(ValueMapAccess {
                iter: entries.into_iter(),
                value: None,
            }),
            other => ValueDeserializer(other).deserialize_any(visitor),
        }
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, ValueError> {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, ValueError> {
        // Not on the hot path for this crate's own types: `Color`/
        // `Gradient`, the only real internally-tagged enums here, go
        // through `deserialize_any` instead (the only thing an
        // internally-/untagged enum's derived `Deserialize` can call on a
        // self-describing format) -- this exists for completeness/any
        // future externally-tagged enum, using the conventional shape a
        // bare string (unit variant) or a single-entry map (variant ->
        // payload) already produced by `serialize_unit_variant`/
        // `serialize_newtype_variant`/etc. above.
        match self.0 {
            Value::Text(variant) => visitor.visit_enum(ValueEnumAccess {
                variant,
                value: None,
            }),
            Value::Map(mut entries) if entries.len() == 1 => {
                let (tag, value) = entries.remove(0);
                let Value::Text(variant) = tag else {
                    return Err(ValueError::custom("expected a string enum variant tag"));
                };
                visitor.visit_enum(ValueEnumAccess {
                    variant,
                    value: Some(value),
                })
            }
            other => Err(ValueError::custom(format!(
                "expected an enum variant (a string or single-entry map), found {other:?}"
            ))),
        }
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
        self.deserialize_any(visitor)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
        self.deserialize_any(visitor)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

struct ValueSeqAccess {
    iter: std::vec::IntoIter<Value>,
}

impl<'de> SeqAccess<'de> for ValueSeqAccess {
    type Error = ValueError;

    fn next_element_seed<T: DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, ValueError> {
        match self.iter.next() {
            Some(value) => seed.deserialize(ValueDeserializer(value)).map(Some),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.iter.len())
    }
}

struct ValueMapAccess {
    iter: std::vec::IntoIter<(Value, Value)>,
    value: Option<Value>,
}

impl<'de> MapAccess<'de> for ValueMapAccess {
    type Error = ValueError;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, ValueError> {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(ValueDeserializer(key)).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, ValueError> {
        let value = self
            .value
            .take()
            .ok_or_else(|| ValueError::custom("next_value_seed called before next_key_seed"))?;
        seed.deserialize(ValueDeserializer(value))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.iter.len())
    }
}

struct ValueEnumAccess {
    variant: String,
    value: Option<Value>,
}

impl<'de> EnumAccess<'de> for ValueEnumAccess {
    type Error = ValueError;
    type Variant = ValueVariantAccess;

    fn variant_seed<S: DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<(S::Value, ValueVariantAccess), ValueError> {
        let variant = seed.deserialize(ValueDeserializer(Value::Text(self.variant)))?;
        Ok((variant, ValueVariantAccess { value: self.value }))
    }
}

struct ValueVariantAccess {
    value: Option<Value>,
}

impl<'de> VariantAccess<'de> for ValueVariantAccess {
    type Error = ValueError;

    fn unit_variant(self) -> Result<(), ValueError> {
        Ok(())
    }

    fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, ValueError> {
        let value = self
            .value
            .ok_or_else(|| ValueError::custom("expected a newtype variant payload"))?;
        seed.deserialize(ValueDeserializer(value))
    }

    fn tuple_variant<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value, ValueError> {
        let value = self
            .value
            .ok_or_else(|| ValueError::custom("expected a tuple variant payload"))?;
        ValueDeserializer(value).deserialize_seq(visitor)
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, ValueError> {
        let value = self
            .value
            .ok_or_else(|| ValueError::custom("expected a struct variant payload"))?;
        ValueDeserializer(value).deserialize_map(visitor)
    }
}
