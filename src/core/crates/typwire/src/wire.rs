use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::{BuildHasher, Hash};

use serde::Serialize;
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::Bytes;

/// A type that is explicitly allowed to cross the Typwire boundary.
///
/// Serde defines the representation while this marker controls which types are
/// part of a plugin's public wire contract.
pub trait Typwire: Serialize + DeserializeOwned {}

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

// CBOR-specific behavior (narrow float widths, tag rejection, unit vs. null
// wire shape, ...) is tested alongside `Cbor` itself in `format/cbor.rs`.
