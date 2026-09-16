use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::{BuildHasher, Hash};

use cbor4ii::core::Value;
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
    let bytes = cbor4ii::serde::to_vec(Vec::new(), value)
        .map_err(|error| Error::Serialization(error.to_string()))?;

    let value: Value = cbor4ii::serde::from_slice(&bytes)
        .map_err(|error| Error::MalformedCbor(error.to_string()))?;
    validate(&value)?;

    Ok(bytes)
}

/// Decode validated CBOR bytes into an explicitly supported owned value.
pub fn decode<T: Typwire>(bytes: &[u8]) -> Result<T, Error> {
    let value: Value = cbor4ii::serde::from_slice(bytes)
        .map_err(|error| Error::MalformedCbor(error.to_string()))?;
    validate(&value)?;

    // `T`'s derived `Deserialize` reads typed scalars (e.g. `deserialize_f64`),
    // which cbor4ii resolves strictly: a `f64` field rejects wire bytes that
    // used the narrower CBOR float widths Typst's encoder picks canonically.
    // Re-encoding the already-parsed `Value` normalizes floats to full width
    // before the typed decode, at the cost of a second pass.
    value_to_owned(value).map_err(Error::Deserialization)
}

fn validate(value: &Value) -> Result<(), Error> {
    match value {
        Value::Integer(integer) => i64::try_from(*integer)
            .map(|_| ())
            .map_err(|_| Error::IntegerOutOfRange),
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

    #[test]
    fn rejects_non_string_map_keys() {
        let value = BTreeMap::from([(1_i64, "value")]);
        let bytes = cbor4ii::serde::to_vec(Vec::new(), &value).unwrap();
        let value: Value = cbor4ii::serde::from_slice(&bytes).unwrap();
        assert_eq!(validate(&value), Err(Error::IncompatibleMapKey));
    }

    #[test]
    fn rejects_out_of_range_integer() {
        // `u64` and `i128` aren't `Typwire` (Typst only has `i64`/`f64`), so
        // this goes through cbor4ii's serializer directly rather than the
        // public `encode`. Still a plain (non-bignum) CBOR unsigned integer,
        // so it decodes into a `Value` that `validate` range-checks.
        let bytes = cbor4ii::serde::to_vec(Vec::new(), &(i64::MAX as u64 + 1)).unwrap();
        assert_eq!(decode::<i64>(&bytes).unwrap_err(), Error::IntegerOutOfRange);
    }

    #[test]
    fn rejects_integer_too_large_for_any_supported_representation() {
        // Exceeds even i128 (cbor4ii encodes it as a CBOR bignum, tag 2, per
        // RFC 8949), so it fails while parsing into `Value` rather than during
        // `validate`. Still rejected, just via a different `Error` variant.
        let bytes = cbor4ii::serde::to_vec(Vec::new(), &u128::MAX).unwrap();
        assert!(decode::<i64>(&bytes).is_err());
    }

    #[test]
    fn rejects_cbor_tags() {
        use cbor4ii::core::enc::Encode;
        use cbor4ii::core::utils::BufWriter;

        // Tag 1 (epoch datetime) is not one of the bignum tags cbor4ii's
        // `Value` decoder special-cases, so it fails to parse into a `Value`
        // at all rather than surfacing as a literal `Value::Tag`.
        let mut writer = BufWriter::new(Vec::new());
        Value::Tag(1, Box::new(Value::Null))
            .encode(&mut writer)
            .unwrap();
        let bytes = writer.into_inner();

        assert!(matches!(
            decode::<()>(&bytes).unwrap_err(),
            Error::MalformedCbor(_)
        ));
    }
}
