use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::{BuildHasher, Hash};

use cbor4ii::core::Value;
use serde::de::{
    DeserializeOwned, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

/// A type that is explicitly allowed to cross the Typwire boundary.
///
/// Serde defines the representation while this marker controls which types are
/// part of a plugin's public wire contract.
pub trait Typwire: Serialize + DeserializeOwned {}

/// An owned Typst byte value.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    pub const fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<Bytes> for Vec<u8> {
    fn from(value: Bytes) -> Self {
        value.0
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BytesVisitor;

        impl<'de> Visitor<'de> for BytesVisitor {
            type Value = Bytes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a CBOR byte string")
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Bytes(value.to_vec()))
            }

            fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Bytes(value))
            }
        }

        deserializer.deserialize_byte_buf(BytesVisitor)
    }
}

/// An error raised while encoding or decoding a supported wire format.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("could not serialize value: {0}")]
    Serialization(String),
    #[error("could not deserialize value: {0}")]
    Deserialization(String),
}

/// A wire format `typwire` can encode/decode a [`Typwire`] value through.
///
/// `#[typwire::export(format = "...")]` and `typwire::<Format>::encode`/
/// `decode` are the two ways to pick a non-default format; [`encode`]/
/// [`decode`] (this module's free functions) always mean [`Cbor`], matching
/// Typst's own `typwire.call` default.
pub trait Format {
    fn encode<T: Typwire>(value: &T) -> Result<Vec<u8>, Error>;
    fn decode<T: Typwire>(bytes: &[u8]) -> Result<T, Error>;
}

/// The default wire format: CBOR, via `cbor4ii`.
pub struct Cbor;

impl Format for Cbor {
    fn encode<T: Typwire>(value: &T) -> Result<Vec<u8>, Error> {
        cbor4ii::serde::to_vec(Vec::new(), value)
            .map_err(|error| Error::Serialization(error.to_string()))
    }

    fn decode<T: Typwire>(bytes: &[u8]) -> Result<T, Error> {
        // `T`'s derived `Deserialize` reads typed scalars (e.g. `deserialize_f64`),
        // which cbor4ii resolves strictly: a bare `f64` field rejects wire bytes
        // that used the narrower CBOR float widths Typst's encoder picks
        // canonically. `LenientDeserializer` redirects only the float-hinted
        // calls through cbor4ii's own width-tolerant `deserialize_any`, so this
        // stays a single parse pass.
        let reader = cbor4ii::core::utils::SliceReader::new(bytes);
        let mut inner = cbor4ii::serde::Deserializer::new(reader);
        T::deserialize(LenientDeserializer(&mut inner))
            .map_err(|error| Error::Deserialization(error.to_string()))
    }
}

/// Encode an explicitly supported value into CBOR bytes.
pub fn encode<T: Typwire>(value: &T) -> Result<Vec<u8>, Error> {
    Cbor::encode(value)
}

/// Decode CBOR bytes into an explicitly supported owned value.
pub fn decode<T: Typwire>(bytes: &[u8]) -> Result<T, Error> {
    Cbor::decode(bytes)
}

/// A `Deserializer` wrapper that widens narrow CBOR floats.
///
/// cbor4ii's typed float decode (`deserialize_f32`/`deserialize_f64`) only
/// accepts the literal 4-/8-byte marker for the target width, rejecting the
/// narrower F16/F32 encodings Typst's `cbor.encode` (itself `ciborium`-based)
/// picks canonically. cbor4ii's own `deserialize_any` already handles every
/// width correctly, so this wrapper redirects just the float-hinted calls
/// there, passing the original visitor straight through unwrapped: serde's
/// built-in float visitors already implement both `visit_f32` and
/// `visit_f64` with widening, so no custom visitor is needed for that part.
///
/// Every other recursion point (`seq`, `map`, `enum`, `option`, ...) is
/// forwarded through a small local `Wrap` visitor that re-applies this same
/// wrapper to whatever nested `Deserializer`/`DeserializeSeed` cbor4ii's own
/// `SeqAccess`/`MapAccess`/`EnumAccess` hand back — those are cbor4ii's own
/// concrete types holding a direct reference to the inner deserializer, so
/// wrapping only the top-level call would not reach nested fields. Wrapping
/// the `DeserializeSeed` at each step (`LenientSeed`) is what makes the
/// leniency apply recursively.
struct LenientDeserializer<D>(D);

impl<'de, D: Deserializer<'de>> Deserializer<'de> for LenientDeserializer<D> {
    type Error = D::Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_any(visitor)
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_any(visitor)
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_any(visitor)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_option(Wrap(visitor))
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_newtype_struct(name, Wrap(visitor))
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_seq(Wrap(visitor))
    }

    fn deserialize_tuple<V: Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_tuple(len, Wrap(visitor))
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_tuple_struct(name, len, Wrap(visitor))
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_map(Wrap(visitor))
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_struct(name, fields, Wrap(visitor))
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_enum(name, variants, Wrap(visitor))
    }

    // Trivial forwards to cbor4ii's own type-hinted decode for everything
    // that isn't a float. Deliberately *not* `forward_to_deserialize_any!`:
    // that macro routes every one of these through `deserialize_any`, which
    // is wrong for `unit`/`unit_struct` specifically -- cbor4ii's
    // `deserialize_any` treats a CBOR null as an Option-shaped value (calls
    // `visit_none`), while the real `deserialize_unit` expects an empty-array
    // marker and calls `visit_unit`; these are different wire shapes that
    // happen to share a byte for other formats but not here.
    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_bool(visitor)
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_i8(visitor)
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_i16(visitor)
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_i32(visitor)
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_i64(visitor)
    }

    fn deserialize_i128<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_i128(visitor)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_u8(visitor)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_u16(visitor)
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_u32(visitor)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_u64(visitor)
    }

    fn deserialize_u128<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_u128(visitor)
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_char(visitor)
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_str(visitor)
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_string(visitor)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_bytes(visitor)
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_byte_buf(visitor)
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_unit(visitor)
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_unit_struct(name, visitor)
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_identifier(visitor)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_ignored_any(visitor)
    }
}

/// A `Visitor` wrapper that re-applies [`LenientDeserializer`]/[`LenientSeed`]
/// to whatever nested deserializer, seq/map/enum access cbor4ii's own
/// visit_* callbacks hand back, then forwards to the real visitor.
struct Wrap<V>(V);

impl<'de, V: Visitor<'de>> Visitor<'de> for Wrap<V> {
    type Value = V::Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.expecting(formatter)
    }

    fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        self.0.visit_none()
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        self.0.visit_some(LenientDeserializer(deserializer))
    }

    fn visit_newtype_struct<D: Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error> {
        self.0
            .visit_newtype_struct(LenientDeserializer(deserializer))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
        self.0.visit_seq(LenientSeqAccess(seq))
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        self.0.visit_map(LenientMapAccess(map))
    }

    fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        self.0.visit_enum(LenientEnumAccess(data))
    }
}

/// A `DeserializeSeed` wrapper that wraps the deserializer it's given with
/// [`LenientDeserializer`] before delegating. This is what makes leniency
/// apply recursively through cbor4ii's own `SeqAccess`/`MapAccess`/
/// `EnumAccess`, which hold a direct reference to the concrete inner
/// deserializer rather than going back through any wrapper.
struct LenientSeed<S>(S);

impl<'de, S: DeserializeSeed<'de>> DeserializeSeed<'de> for LenientSeed<S> {
    type Value = S::Value;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        self.0.deserialize(LenientDeserializer(deserializer))
    }
}

struct LenientSeqAccess<A>(A);

impl<'de, A: SeqAccess<'de>> SeqAccess<'de> for LenientSeqAccess<A> {
    type Error = A::Error;

    fn next_element_seed<S: DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<Option<S::Value>, Self::Error> {
        self.0.next_element_seed(LenientSeed(seed))
    }

    fn size_hint(&self) -> Option<usize> {
        self.0.size_hint()
    }
}

struct LenientMapAccess<A>(A);

impl<'de, A: MapAccess<'de>> MapAccess<'de> for LenientMapAccess<A> {
    type Error = A::Error;

    fn next_key_seed<S: DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<Option<S::Value>, Self::Error> {
        self.0.next_key_seed(LenientSeed(seed))
    }

    fn next_value_seed<S: DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<S::Value, Self::Error> {
        self.0.next_value_seed(LenientSeed(seed))
    }

    fn size_hint(&self) -> Option<usize> {
        self.0.size_hint()
    }
}

struct LenientEnumAccess<A>(A);

impl<'de, A: EnumAccess<'de>> EnumAccess<'de> for LenientEnumAccess<A> {
    type Error = A::Error;
    type Variant = LenientVariantAccess<A::Variant>;

    fn variant_seed<S: DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<(S::Value, Self::Variant), Self::Error> {
        self.0
            .variant_seed(LenientSeed(seed))
            .map(|(value, variant)| (value, LenientVariantAccess(variant)))
    }
}

struct LenientVariantAccess<A>(A);

impl<'de, A: VariantAccess<'de>> VariantAccess<'de> for LenientVariantAccess<A> {
    type Error = A::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        self.0.unit_variant()
    }

    fn newtype_variant_seed<S: DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<S::Value, Self::Error> {
        self.0.newtype_variant_seed(LenientSeed(seed))
    }

    fn tuple_variant<V: Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.tuple_variant(len, Wrap(visitor))
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.struct_variant(fields, Wrap(visitor))
    }
}

macro_rules! impl_typwire {
    ($($ty:ty),+ $(,)?) => {
        $(impl Typwire for $ty {})+
    };
}

impl_typwire!(
    (),
    bool,
    char,
    i64,
    // Not a Typst-native number type; kept only so `Vec<u8>` (via the blanket
    // `Typwire for Vec<T>` impl below) can represent a plain byte array,
    // distinct from `Bytes`.
    u8,
    f64,
    String,
    Bytes
);

impl<T: Typwire> Typwire for Option<T> {}
impl<T: Typwire> Typwire for Vec<T> {}
impl<T: Typwire> Typwire for VecDeque<T> {}
impl<T: Typwire> Typwire for LinkedList<T> {}
impl<T: Typwire + Ord> Typwire for BTreeSet<T> {}
impl<T: Typwire + Eq + Hash> Typwire for HashSet<T> {}
impl<T: Typwire, const N: usize> Typwire for [T; N] where [T; N]: Serialize + DeserializeOwned {}
impl<T: Typwire> Typwire for Box<T> {}
impl<T: Typwire> Typwire for BTreeMap<String, T> {}
impl<T: Typwire, S> Typwire for HashMap<String, T, S>
where
    S: BuildHasher + Default,
    HashMap<String, T, S>: Serialize + DeserializeOwned,
{
}

macro_rules! tuple_impls {
    ($(($($name:ident),+)),+ $(,)?) => {
        $(
            impl<$($name: Typwire),+> Typwire for ($($name,)+) {}
        )+
    };
}

tuple_impls!(
    (A),
    (A, B),
    (A, B, C),
    (A, B, C, D),
    (A, B, C, D, E),
    (A, B, C, D, E, F),
    (A, B, C, D, E, F, G),
    (A, B, C, D, E, F, G, H),
    (A, B, C, D, E, F, G, H, I),
    (A, B, C, D, E, F, G, H, I, J),
    (A, B, C, D, E, F, G, H, I, J, K),
    (A, B, C, D, E, F, G, H, I, J, K, L),
);

pub(crate) fn value_to_owned<T: DeserializeOwned>(value: Value) -> Result<T, String> {
    let bytes = cbor4ii::serde::to_vec(Vec::new(), &value).map_err(|error| error.to_string())?;
    cbor4ii::serde::from_slice(&bytes).map_err(|error| error.to_string())
}

/// The mirror of [`value_to_owned`]: turn any `Typwire` value into a generic
/// [`Value`] tree, round-tripping through real CBOR bytes the same way
/// `value_to_owned` does in the other direction. Used by the text formats
/// (`Json`/`Yaml`/`Toml`) to get a tree they can rewrite [`Bytes`] nodes in
/// before handing it to a format that has no native byte-string concept --
/// see `rewrite_bytes_to_array`. `Cbor` never needs this: it serializes
/// `T` directly in one pass, `Bytes` included, exactly as today.
#[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
pub(crate) fn value_from_owned<T: Serialize>(value: &T) -> Result<Value, String> {
    let bytes = cbor4ii::serde::to_vec(Vec::new(), value).map_err(|error| error.to_string())?;
    cbor4ii::serde::from_slice(&bytes).map_err(|error| error.to_string())
}

/// Replace every [`Value::Bytes`] node in `value` with the same tagged
/// dictionary shape the Typst side uses for a format with no native
/// byte-string type (`src/wire/rewrite-bytes.typ`): `{typwire-type:
/// "bytes", data: [...]}`, `data` a plain array of byte integers -- not
/// base64, since Typst has no built-in base64 codec to match it against.
#[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
pub(crate) fn rewrite_bytes_to_array(value: &mut Value) {
    match value {
        Value::Bytes(data) => {
            let entries = vec![
                (
                    Value::Text("typwire-type".to_owned()),
                    Value::Text("bytes".to_owned()),
                ),
                (
                    Value::Text("data".to_owned()),
                    Value::Array(
                        data.iter()
                            .map(|byte| Value::Integer(*byte as i128))
                            .collect(),
                    ),
                ),
            ];
            *value = Value::Map(entries);
        }
        Value::Array(items) => items.iter_mut().for_each(rewrite_bytes_to_array),
        Value::Map(entries) => entries
            .iter_mut()
            .for_each(|(_, entry)| rewrite_bytes_to_array(entry)),
        _ => {}
    }
}

/// The inverse of [`rewrite_bytes_to_array`]: replace every `{typwire-type:
/// "bytes", data: [...]}` tagged dictionary with a real [`Value::Bytes`].
#[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
pub(crate) fn rewrite_array_to_bytes(value: &mut Value) {
    match value {
        Value::Map(entries) => {
            let is_tagged_bytes = entries.iter().any(|(key, entry)| {
                matches!(
                    (key, entry),
                    (Value::Text(key), Value::Text(tag))
                        if key == "typwire-type" && tag == "bytes"
                )
            });

            if is_tagged_bytes {
                let data = entries
                    .iter()
                    .find_map(|(key, entry)| match (key, entry) {
                        (Value::Text(key), Value::Array(items)) if key == "data" => {
                            Some(items.clone())
                        }
                        _ => None,
                    })
                    .unwrap_or_default();
                let bytes = data
                    .into_iter()
                    .filter_map(|item| match item {
                        Value::Integer(byte) => u8::try_from(byte).ok(),
                        _ => None,
                    })
                    .collect();
                *value = Value::Bytes(bytes);
            } else {
                entries
                    .iter_mut()
                    .for_each(|(_, entry)| rewrite_array_to_bytes(entry));
            }
        }
        Value::Array(items) => items.iter_mut().for_each(rewrite_array_to_bytes),
        _ => {}
    }
}

pub(crate) fn invalid_value<E: serde::de::Error>(message: impl std::fmt::Display) -> E {
    E::custom(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_use_a_byte_string_but_vec_uses_an_array() {
        let bytes = encode(&Bytes::new(vec![1, 2])).unwrap();
        let vector = encode(&vec![1_u8, 2]).unwrap();

        assert!(matches!(
            cbor4ii::serde::from_slice::<Value>(&bytes).unwrap(),
            Value::Bytes(_)
        ));
        assert!(matches!(
            cbor4ii::serde::from_slice::<Value>(&vector).unwrap(),
            Value::Array(_)
        ));
    }

    // `wire::validate` was removed: its three checks (integer fits i64,
    // string-only map keys, no CBOR tags) were unreachable through any
    // type-safe path on either side of the boundary (Typst's own encoder
    // can't produce them; `Typwire` is only implemented for `i64` and
    // string-keyed maps). The following tests confirm cbor4ii's own decode
    // still fails closed on the same malformed inputs without that pass —
    // the safety argument for dropping it — just via a generic
    // `Error::Deserialization` rather than a dedicated variant.

    #[test]
    fn rejects_non_string_map_keys() {
        let value = BTreeMap::from([(1_i64, "value")]);
        let bytes = cbor4ii::serde::to_vec(Vec::new(), &value).unwrap();
        assert!(decode::<BTreeMap<String, String>>(&bytes).is_err());
    }

    #[test]
    fn rejects_out_of_range_integer() {
        // `u64` and `i128` aren't `Typwire` (Typst only has `i64`/`f64`), so
        // this goes through cbor4ii's serializer directly rather than the
        // public `encode`. cbor4ii's typed integer decode uses a checked
        // `TryFrom` internally, so it still fails closed without `validate`.
        let bytes = cbor4ii::serde::to_vec(Vec::new(), &(i64::MAX as u64 + 1)).unwrap();
        assert!(decode::<i64>(&bytes).is_err());
    }

    #[test]
    fn rejects_integer_too_large_for_any_supported_representation() {
        // Exceeds even i128 (cbor4ii encodes it as a CBOR bignum, tag 2, per
        // RFC 8949), so it fails while parsing rather than during a typed
        // integer conversion. Still rejected either way.
        let bytes = cbor4ii::serde::to_vec(Vec::new(), &u128::MAX).unwrap();
        assert!(decode::<i64>(&bytes).is_err());
    }

    #[test]
    fn rejects_cbor_tags() {
        use cbor4ii::core::enc::Encode;
        use cbor4ii::core::utils::BufWriter;

        // Tag 1 (epoch datetime) is not one of the bignum tags cbor4ii's
        // decode special-cases, so it hard-errors rather than silently
        // unwrapping to the tagged value.
        let mut writer = BufWriter::new(Vec::new());
        Value::Tag(1, Box::new(Value::Null))
            .encode(&mut writer)
            .unwrap();
        let bytes = writer.into_inner();

        assert!(decode::<()>(&bytes).is_err());
    }

    #[test]
    fn narrow_width_float_fields_round_trip() {
        // Regression test for the bug `LenientDeserializer` exists to fix:
        // cbor4ii's typed `deserialize_f64` rejects a narrower CBOR float
        // marker (F16/F32) even when the target is `f64`, which is exactly
        // what Typst's `cbor.encode` (itself `ciborium`-based) picks
        // canonically for a "round" value like `0.0`.
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Struct {
            value: f64,
        }
        impl Typwire for Struct {}

        // Hand-crafted CBOR for `{"value": 0.0}` using a half-precision
        // (F16) float marker instead of the full 8-byte F64 marker.
        let bytes: &[u8] = &[
            0xA1, // map(1)
            0x65, b'v', b'a', b'l', b'u', b'e', // text(5) "value"
            0xF9, 0x00, 0x00, // float16(0.0)
        ];

        assert_eq!(decode::<Struct>(bytes).unwrap(), Struct { value: 0.0 });
    }

    #[test]
    fn narrow_width_float_inside_enum_variant_round_trips() {
        // `Color`/`Gradient` are internally tagged enums
        // (`#[serde(tag = "typwire-type")]`) with `f64`-backed fields inside
        // their variants (`Ratio`, here), so this exercises
        // `deserialize_enum`'s wrapping specifically, not just plain struct
        // fields.
        let black = crate::Color::Luma(crate::Luma::new(
            crate::Ratio::new(0.0),
            crate::Ratio::new(1.0),
        ));
        let bytes = encode(&black).unwrap();
        assert_eq!(decode::<crate::Color>(&bytes).unwrap(), black);
    }

    #[test]
    fn unit_round_trips() {
        // Regression test for a real bug caught while adding coverage: an
        // earlier version of `LenientDeserializer` routed `unit`/
        // `unit_struct` through `deserialize_any` (via
        // `forward_to_deserialize_any!`), but cbor4ii's `deserialize_any`
        // treats a CBOR null as Option-shaped (calls `visit_none`), while
        // `()`'s real wire shape is a zero-length array and needs the
        // strict `deserialize_unit` (calls `visit_unit`). Routing through
        // `deserialize_any` made every `()` decode fail with "invalid type:
        // Option value, expected unit".
        let bytes = encode(&()).unwrap();
        assert_eq!(decode::<()>(&bytes).unwrap(), ());
    }

    #[test]
    fn unit_is_not_the_same_wire_shape_as_a_cbor_null() {
        // `()` is a zero-length array, not `none`'s CBOR null -- pinning
        // down the distinction the previous test's regression was about.
        let bytes = cbor4ii::serde::to_vec(Vec::new(), &Option::<i64>::None).unwrap();
        assert!(decode::<()>(&bytes).is_err());
    }
}
