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
/// [`decode`] (this module's free functions) always mean `Cbor`
/// (`crate::format::Cbor`), matching Typst's own `typwire.call` default.
pub trait Format {
    fn encode<T: Typwire>(value: &T) -> Result<Vec<u8>, Error>;
    fn decode<T: Typwire>(bytes: &[u8]) -> Result<T, Error>;
}

/// Encode an explicitly supported value into CBOR bytes.
pub fn encode<T: Typwire>(value: &T) -> Result<Vec<u8>, Error> {
    crate::format::Cbor::encode(value)
}

/// Decode CBOR bytes into an explicitly supported owned value.
pub fn decode<T: Typwire>(bytes: &[u8]) -> Result<T, Error> {
    crate::format::Cbor::decode(bytes)
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

/// Reconstruct any `Typwire` value directly from a generic [`Value`] tree,
/// with no CBOR bytes in between -- see `format/value.rs::from_value`.
/// Called both by `Any` (`any.rs`, to reconstruct a native type nested
/// inside it, needed in every build, `Cbor`-only included) and by
/// `Json::decode` (`value_from_owned`'s mirror, below).
pub(crate) fn value_to_owned<T: DeserializeOwned>(value: Value) -> Result<T, String> {
    crate::format::from_value(value).map_err(|error| error.to_string())
}

/// The mirror of [`value_to_owned`]: turn any `Typwire` value into a generic
/// [`Value`] tree directly, with no CBOR bytes in between -- see
/// `format/value.rs::to_value`. Used by `Json` to get a tree it can rewrite
/// [`Bytes`] nodes in before handing it to a format that has no native
/// byte-string concept -- see `rewrite_bytes_to_array`. `Cbor` never needs
/// this: it serializes `T` directly in one pass, `Bytes` included, exactly
/// as today.
#[cfg(feature = "json")]
pub(crate) fn value_from_owned<T: Serialize>(value: &T) -> Result<Value, String> {
    crate::format::to_value(value).map_err(|error| error.to_string())
}

/// Replace every [`Value::Bytes`] node in `value` with the same tagged
/// dictionary shape the Typst side uses for a format with no native
/// byte-string type (`src/wire/types/foundations/bytes.typ`): `{typwire-type:
/// "bytes", data: [...]}`, `data` a plain array of byte integers -- not
/// base64, since Typst has no built-in base64 codec to match it against.
#[cfg(feature = "json")]
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
#[cfg(feature = "json")]
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

// CBOR-specific behavior (narrow float widths, tag rejection, unit vs. null
// wire shape, ...) is tested alongside `Cbor` itself in `format/cbor.rs`.
