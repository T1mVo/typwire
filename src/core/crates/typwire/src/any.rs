use std::collections::BTreeMap;

use ciborium::Value;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    Angle, Bytes, Color, DateTime, Duration, Gradient, Length, Ratio, Type, Typwire, Version,
    wire::{invalid_value, value_to_owned},
};

/// A dynamic value supported by the Typwire protocol.
#[derive(Clone, Debug)]
pub enum Any {
    None,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Bytes(Bytes),
    Array(Vec<Self>),
    Dictionary(BTreeMap<String, Self>),
    Angle(Angle),
    Length(Length),
    Ratio(Ratio),
    Color(Color),
    Gradient(Gradient),
    DateTime(DateTime),
    Duration(Duration),
    Version(Version),
    Type(Type),
}

impl Serialize for Any {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_none(),
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
        }
    }
}

impl<'de> Deserialize<'de> for Any {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        from_value(value).map_err(invalid_value)
    }
}

impl Typwire for Any {}

fn from_value(value: Value) -> Result<Any, String> {
    match value {
        Value::Null => Ok(Any::None),
        Value::Bool(value) => Ok(Any::Bool(value)),
        Value::Integer(value) => i64::try_from(i128::from(value))
            .map(Any::Integer)
            .map_err(|_| "integer is outside Typst's signed 64-bit range".to_owned()),
        Value::Float(value) => Ok(Any::Float(value)),
        Value::Bytes(value) => Ok(Any::Bytes(Bytes::new(value))),
        Value::Text(value) => Ok(Any::String(value)),
        Value::Array(values) => values
            .into_iter()
            .map(from_value)
            .collect::<Result<_, _>>()
            .map(Any::Array),
        Value::Map(entries) => from_map(entries),
        Value::Tag(_, _) => Err("CBOR tags are not supported by Typwire".to_owned()),
        _ => Err("unknown CBOR value is not supported by Typwire".to_owned()),
    }
}

fn from_map(entries: Vec<(Value, Value)>) -> Result<Any, String> {
    let tag = entries.iter().find_map(|(key, value)| match (key, value) {
        (Value::Text(key), Value::Text(value)) if key == "typwire-type" => Some(value.clone()),
        _ => None,
    });

    if let Some(tag) = tag {
        let value = Value::Map(entries);
        return match tag.as_str() {
            "angle" => value_to_owned(value).map(Any::Angle),
            "length" => value_to_owned(value).map(Any::Length),
            "ratio" => value_to_owned(value).map(Any::Ratio),
            "color-luma" | "color-oklab" | "color-oklch" | "color-linear-rgb" | "color-rgb"
            | "color-cmyk" | "color-hsl" | "color-hsv" => value_to_owned(value).map(Any::Color),
            "gradient-linear" | "gradient-radial" | "gradient-conic" => {
                value_to_owned(value).map(Any::Gradient)
            }
            "datetime" => value_to_owned(value).map(Any::DateTime),
            "duration" => value_to_owned(value).map(Any::Duration),
            "version" => value_to_owned(value).map(Any::Version),
            "type" => value_to_owned(value).map(Any::Type),
            other => Err(format!("unknown typwire-type tag `{other}`")),
        };
    }

    entries
        .into_iter()
        .map(|(key, value)| {
            let Value::Text(key) = key else {
                return Err("Typwire dictionaries require string keys".to_owned());
            };
            Ok((key, from_value(value)?))
        })
        .collect::<Result<_, _>>()
        .map(Any::Dictionary)
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
);

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
