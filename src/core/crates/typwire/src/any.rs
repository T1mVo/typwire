use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;

use serde::de::value::MapAccessDeserializer;
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "decimal")]
use crate::Decimal;
#[cfg(feature = "regex")]
use crate::Regex;
use crate::{
    Alignment, Angle, Arguments, Auto, Bytes, Color, DateTime, Duration, Gradient, Label, Length,
    Path, Ratio, Stroke, Symbol, Type, Typwire, Version,
    tag::{TAG_KEY, WithTag},
};

/// A dynamic value supported by the Typwire protocol.
#[derive(Clone, Debug)]
pub enum Any {
    /// Typst `none`.
    None,
    /// Typst `auto`.
    Auto,
    /// A Typst `bool`.
    Bool(bool),
    /// A Typst `int`.
    Integer(i64),
    /// A Typst `float`.
    Float(f64),
    /// A Typst `str`.
    String(String),
    /// Typst `bytes`.
    Bytes(Bytes),
    /// A Typst `array` of dynamic values.
    Array(Vec<Self>),
    /// A Typst `dictionary` with string keys.
    Dictionary(BTreeMap<String, Self>),
    /// A Typst `angle`.
    Angle(Angle),
    /// A Typst `length`.
    Length(Length),
    /// A Typst `ratio`.
    Ratio(Ratio),
    /// A Typst `color`, in any of its color spaces.
    Color(Color),
    /// A Typst `gradient`.
    Gradient(Gradient),
    /// A Typst `datetime`.
    DateTime(DateTime),
    /// A Typst `duration`.
    Duration(Duration),
    /// A Typst `version`.
    Version(Version),
    /// A Typst `type`.
    Type(Type),
    /// A Typst `path`.
    Path(Path),
    /// A Typst `symbol`, including its modifier variants.
    Symbol(Symbol),
    /// Typst `arguments`.
    Arguments(Arguments),
    /// A Typst `label`.
    Label(Label),
    /// A Typst `alignment`.
    Alignment(Alignment),
    /// A Typst `stroke`.
    Stroke(Stroke),
    #[cfg(feature = "regex")]
    /// A Typst `regex`.
    Regex(Regex),
    #[cfg(feature = "decimal")]
    /// A Typst `decimal`.
    Decimal(Decimal),
}

impl Serialize for Any {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_none(),
            Self::Auto => Auto.serialize(serializer),
            Self::Bool(value) => value.serialize(serializer),
            Self::Integer(value) => value.serialize(serializer),
            Self::Float(value) => value.serialize(serializer),
            Self::String(value) => value.serialize(serializer),
            Self::Bytes(value) => value.serialize(serializer),
            Self::Array(value) => value.serialize(serializer),
            Self::Dictionary(value) => value.serialize(serializer),
            Self::Angle(value) => value.serialize(serializer),
            Self::Length(value) => value.serialize(serializer),
            Self::Ratio(value) => value.serialize(serializer),
            Self::Color(value) => value.serialize(serializer),
            Self::Gradient(value) => value.serialize(serializer),
            Self::DateTime(value) => value.serialize(serializer),
            Self::Duration(value) => value.serialize(serializer),
            Self::Version(value) => value.serialize(serializer),
            Self::Type(value) => value.serialize(serializer),
            Self::Path(value) => value.serialize(serializer),
            Self::Symbol(value) => value.serialize(serializer),
            Self::Arguments(value) => value.serialize(serializer),
            Self::Label(value) => value.serialize(serializer),
            Self::Alignment(value) => value.serialize(serializer),
            Self::Stroke(value) => value.serialize(serializer),
            #[cfg(feature = "regex")]
            Self::Regex(value) => value.serialize(serializer),
            #[cfg(feature = "decimal")]
            Self::Decimal(value) => value.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Any {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(AnyVisitor)
    }
}

impl Typwire for Any {}

/// Builds an `Any` straight from the decoder's own callbacks, in one pass
/// -- no intermediate `cbor4ii::core::Value` tree to build first and walk
/// a second time. A map whose first key is `typwire-type` is handed to the
/// native type its tag names (see `crate::tag::next_tag` for why the tag
/// has to come first); any other map is a plain dictionary.
struct AnyVisitor;

const OUT_OF_RANGE: &str = "integer is outside Typst's signed 64-bit range";

impl<'de> Visitor<'de> for AnyVisitor {
    type Value = Any;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Typwire value")
    }

    fn visit_bool<E: de::Error>(self, value: bool) -> Result<Any, E> {
        Ok(Any::Bool(value))
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Any, E> {
        Ok(Any::Integer(value))
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Any, E> {
        i64::try_from(value)
            .map(Any::Integer)
            .map_err(|_| E::custom(OUT_OF_RANGE))
    }

    fn visit_i128<E: de::Error>(self, value: i128) -> Result<Any, E> {
        i64::try_from(value)
            .map(Any::Integer)
            .map_err(|_| E::custom(OUT_OF_RANGE))
    }

    fn visit_u128<E: de::Error>(self, value: u128) -> Result<Any, E> {
        i64::try_from(value)
            .map(Any::Integer)
            .map_err(|_| E::custom(OUT_OF_RANGE))
    }

    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Any, E> {
        Ok(Any::Float(value))
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Any, E> {
        Ok(Any::String(value.to_owned()))
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<Any, E> {
        Ok(Any::String(value))
    }

    fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<Any, E> {
        Ok(Any::Bytes(Bytes::new(value.to_vec())))
    }

    fn visit_byte_buf<E: de::Error>(self, value: Vec<u8>) -> Result<Any, E> {
        Ok(Any::Bytes(Bytes::new(value)))
    }

    // CBOR's `null` arrives as `visit_none`, `format/value.rs`'s (JSON's)
    // as `visit_unit`.
    fn visit_none<E: de::Error>(self) -> Result<Any, E> {
        Ok(Any::None)
    }

    fn visit_unit<E: de::Error>(self) -> Result<Any, E> {
        Ok(Any::None)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Any, D::Error> {
        Any::deserialize(deserializer)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Any, A::Error> {
        // Capped, since the hint comes straight from the (untrusted) input.
        let mut values = Vec::with_capacity(seq.size_hint().unwrap_or(0).min(4096));
        while let Some(value) = seq.next_element()? {
            values.push(value);
        }
        Ok(Any::Array(values))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Any, A::Error> {
        let Some(first) = map.next_key_seed(StrSeed)? else {
            return Ok(Any::Dictionary(BTreeMap::new()));
        };

        if first == TAG_KEY {
            let tag = map.next_value_seed(StrSeed)?;
            return native(&tag, map);
        }

        let mut entries = BTreeMap::new();
        entries.insert(first.into_owned(), map.next_value()?);
        while let Some((key, value)) = map.next_entry()? {
            entries.insert(key, value);
        }
        Ok(Any::Dictionary(entries))
    }
}

/// Decode the rest of a tagged map (its tag already read) as the native type
/// `tag` names.
fn native<'de, A: MapAccess<'de>>(tag: &str, map: A) -> Result<Any, A::Error> {
    fn read<'de, T: Deserialize<'de>, A: MapAccess<'de>>(tag: &str, map: A) -> Result<T, A::Error> {
        T::deserialize(MapAccessDeserializer::new(WithTag::new(tag, map)))
    }

    match tag {
        "angle" => read(tag, map).map(Any::Angle),
        "length" => read(tag, map).map(Any::Length),
        "ratio" => read(tag, map).map(Any::Ratio),
        "color-luma" | "color-oklab" | "color-oklch" | "color-linear-rgb" | "color-rgb"
        | "color-cmyk" | "color-hsl" | "color-hsv" => read(tag, map).map(Any::Color),
        "gradient-linear" | "gradient-radial" | "gradient-conic" => {
            read(tag, map).map(Any::Gradient)
        }
        "auto" => read::<Auto, _>(tag, map).map(|_| Any::Auto),
        "datetime" => read(tag, map).map(Any::DateTime),
        "duration" => read(tag, map).map(Any::Duration),
        "version" => read(tag, map).map(Any::Version),
        "type" => read(tag, map).map(Any::Type),
        "path" => read(tag, map).map(Any::Path),
        "symbol" => read(tag, map).map(Any::Symbol),
        "arguments" => read(tag, map).map(Any::Arguments),
        "label" => read(tag, map).map(Any::Label),
        "alignment" => read(tag, map).map(Any::Alignment),
        "stroke" => read(tag, map).map(Any::Stroke),
        #[cfg(feature = "regex")]
        "regex" => read(tag, map).map(Any::Regex),
        #[cfg(feature = "decimal")]
        "decimal" => read(tag, map).map(Any::Decimal),
        other => Err(de::Error::custom(format_args!(
            "unknown typwire-type tag `{other}`"
        ))),
    }
}

/// A string, borrowed from the input when the decoder can (CBOR from a
/// slice always can) -- so comparing a map key against `typwire-type`, or
/// reading a tag, allocates nothing.
struct StrSeed;

impl<'de> DeserializeSeed<'de> for StrSeed {
    type Value = Cow<'de, str>;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_str(self)
    }
}

impl<'de> Visitor<'de> for StrSeed {
    type Value = Cow<'de, str>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_borrowed_str<E: de::Error>(self, value: &'de str) -> Result<Self::Value, E> {
        Ok(Cow::Borrowed(value))
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Cow::Owned(value.to_owned()))
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<Self::Value, E> {
        Ok(Cow::Owned(value))
    }
}

impl From<bool> for Any {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i64> for Any {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for Any {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<String> for Any {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Any {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<Bytes> for Any {
    fn from(value: Bytes) -> Self {
        Self::Bytes(value)
    }
}

impl From<Vec<Any>> for Any {
    fn from(value: Vec<Any>) -> Self {
        Self::Array(value)
    }
}

impl From<BTreeMap<String, Any>> for Any {
    fn from(value: BTreeMap<String, Any>) -> Self {
        Self::Dictionary(value)
    }
}

impl From<Auto> for Any {
    fn from(_: Auto) -> Self {
        Self::Auto
    }
}

macro_rules! native_conversions {
    ($(($ty:ty, $variant:ident)),+ $(,)?) => {
        $(
            impl From<$ty> for Any {
                fn from(value: $ty) -> Self {
                    Self::$variant(value)
                }
            }
        )+
    };
}

native_conversions!(
    (Angle, Angle),
    (Length, Length),
    (Ratio, Ratio),
    (Color, Color),
    (Gradient, Gradient),
    (DateTime, DateTime),
    (Duration, Duration),
    (Version, Version),
    (Type, Type),
    (Path, Path),
    (Symbol, Symbol),
    (Arguments, Arguments),
    (Label, Label),
    (Alignment, Alignment),
    (Stroke, Stroke),
);

#[cfg(feature = "regex")]
impl From<Regex> for Any {
    fn from(value: Regex) -> Self {
        Self::Regex(value)
    }
}

#[cfg(feature = "decimal")]
impl From<Decimal> for Any {
    fn from(value: Decimal) -> Self {
        Self::Decimal(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode, encode};

    use super::*;

    #[test]
    fn tagged_values_are_native_before_dictionary() {
        let bytes = encode(&Length::new(12.0)).unwrap();
        let value: Any = decode(&bytes).unwrap();
        assert!(matches!(value, Any::Length(length) if length.pt() == 12.0));
    }

    #[test]
    fn only_a_leading_tag_makes_a_native_value() {
        // `encode` writes a `BTreeMap`'s keys sorted, so `"a"` lands before
        // the tag -- an ordinary dictionary that happens to contain one.
        let value = BTreeMap::from([
            ("a".to_owned(), Any::from(1_i64)),
            ("typwire-type".to_owned(), Any::from("length")),
            ("points".to_owned(), Any::from(1.0)),
        ]);
        let bytes = encode(&Any::Dictionary(value)).unwrap();
        assert!(matches!(decode::<Any>(&bytes).unwrap(), Any::Dictionary(_)));
    }

    #[test]
    fn narrow_width_float_inside_a_nested_native_type_round_trips() {
        // `Any` decodes through `deserialize_any`, which `LenientDeserializer`
        // has to wrap like every other recursion point for the typed `f64`
        // inside the `Length` it dispatches to to accept Typst's F16 `0.0`.
        let bytes: &[u8] = &[
            0x81, // array(1)
            0xA2, // map(2)
            0x6C, b't', b'y', b'p', b'w', b'i', b'r', b'e', b'-', b't', b'y', b'p', b'e', 0x66,
            b'l', b'e', b'n', b'g', b't', b'h', // "length"
            0x66, b'p', b'o', b'i', b'n', b't', b's', // "points"
            0xF9, 0x00, 0x00, // float16(0.0)
        ];
        let value: Any = decode(bytes).unwrap();
        assert!(
            matches!(&value, Any::Array(items) if matches!(&items[..], [Any::Length(length)] if length.pt() == 0.0))
        );
    }

    #[test]
    fn unknown_typwire_tag_is_rejected() {
        let value = BTreeMap::from([
            ("typwire-type".to_owned(), Any::from("future")),
            ("value".to_owned(), Any::from(1_i64)),
        ]);
        let bytes = encode(&Any::Dictionary(value)).unwrap();
        assert!(
            decode::<Any>(&bytes)
                .unwrap_err()
                .to_string()
                .contains("unknown typwire-type tag")
        );
    }
}
