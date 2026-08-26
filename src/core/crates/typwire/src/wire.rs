use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::{BuildHasher, Hash};

use ciborium::Value;
use serde::de::{DeserializeOwned, Visitor};
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

/// An error raised while encoding or decoding the supported wire subset.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("could not serialize value as CBOR: {0}")]
    Serialization(String),
    #[error("malformed CBOR: {0}")]
    MalformedCbor(String),
    #[error("could not deserialize CBOR value: {0}")]
    Deserialization(String),
    #[error("unsupported CBOR wire shape: {0}")]
    UnsupportedWireShape(&'static str),
    #[error("integer is outside Typst's signed 64-bit range")]
    IntegerOutOfRange,
    #[error("Typwire dictionaries require string keys")]
    IncompatibleMapKey,
}

/// Encode an explicitly supported value into validated CBOR bytes.
pub fn encode<T: Typwire>(value: &T) -> Result<Vec<u8>, Error> {
    let value =
        Value::serialized(value).map_err(|error| Error::Serialization(error.to_string()))?;
    validate(&value)?;

    let mut bytes = Vec::new();
    ciborium::into_writer(&value, &mut bytes)
        .map_err(|error| Error::Serialization(error.to_string()))?;
    Ok(bytes)
}

/// Decode validated CBOR bytes into an explicitly supported owned value.
pub fn decode<T: Typwire>(bytes: &[u8]) -> Result<T, Error> {
    let value: Value =
        ciborium::from_reader(bytes).map_err(|error| Error::MalformedCbor(error.to_string()))?;
    validate(&value)?;
    value
        .deserialized()
        .map_err(|error| Error::Deserialization(error.to_string()))
}

fn validate(value: &Value) -> Result<(), Error> {
    match value {
        Value::Integer(integer) => {
            let integer = i128::from(*integer);
            i64::try_from(integer)
                .map(|_| ())
                .map_err(|_| Error::IntegerOutOfRange)
        }
        Value::Array(values) => values.iter().try_for_each(validate),
        Value::Map(entries) => entries.iter().try_for_each(|(key, value)| {
            if !matches!(key, Value::Text(_)) {
                return Err(Error::IncompatibleMapKey);
            }
            validate(value)
        }),
        Value::Tag(2 | 3, _) => Err(Error::IntegerOutOfRange),
        Value::Tag(_, _) => Err(Error::UnsupportedWireShape("CBOR tags")),
        Value::Null | Value::Bool(_) | Value::Float(_) | Value::Bytes(_) | Value::Text(_) => Ok(()),
        _ => Err(Error::UnsupportedWireShape("unknown CBOR value")),
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
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
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
    value.deserialized().map_err(|error| error.to_string())
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
            ciborium::from_reader::<Value, _>(bytes.as_slice()).unwrap(),
            Value::Bytes(_)
        ));
        assert!(matches!(
            ciborium::from_reader::<Value, _>(vector.as_slice()).unwrap(),
            Value::Array(_)
        ));
    }

    #[test]
    fn rejects_non_string_map_keys() {
        let value = BTreeMap::from([(1_i64, "value")]);
        let error = Value::serialized(&value).unwrap();
        assert_eq!(validate(&error), Err(Error::IncompatibleMapKey));
    }

    #[test]
    fn rejects_out_of_range_integer() {
        assert_eq!(encode(&u128::MAX).unwrap_err(), Error::IntegerOutOfRange);
    }

    #[test]
    fn rejects_cbor_tags() {
        let mut bytes = Vec::new();
        ciborium::into_writer(&Value::Tag(1, Box::new(Value::Null)), &mut bytes).unwrap();
        assert_eq!(
            decode::<()>(&bytes).unwrap_err(),
            Error::UnsupportedWireShape("CBOR tags")
        );
    }
}
