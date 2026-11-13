use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A sequence of bytes, conceptually an array of integers between 0 and 255 but
/// represented far more efficiently.
///
/// Sent as a native byte string in CBOR and as a tagged integer array in JSON,
/// which has no byte type.
///
/// See the [Typst documentation](https://typst.app/docs/reference/foundations/bytes/).
///
/// # Examples
///
/// ```
/// use typwire::{Bytes, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn checksum(data: Bytes) -> Result<i64, String> {
///     Ok(data.as_slice().iter().map(|&byte| i64::from(byte)).sum())
/// }
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    /// Wrap an owned byte vector.
    pub const fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Unwrap into the underlying byte vector.
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }

    /// Borrow the underlying bytes.
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
