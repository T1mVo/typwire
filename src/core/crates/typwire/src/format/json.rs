use cbor4ii::core::Value;

use crate::Typwire;
use crate::wire::{
    Error, Format, rewrite_array_to_bytes, rewrite_bytes_to_array, value_from_owned, value_to_owned,
};

/// The `json` wire format, backed by `serde_json`.
///
/// `Bytes` has no native JSON representation, so `encode`/`decode` round-trip
/// the value through a generic [`Value`] tree first and rewrite any
/// [`Value::Bytes`] node into the same tagged array shape the Typst side
/// uses (`src/wire/rewrite-bytes.typ`) -- `Cbor` needs none of this, since
/// CBOR has a native byte-string type.
pub struct Json;

impl Format for Json {
    fn encode<T: Typwire>(value: &T) -> Result<Vec<u8>, Error> {
        let mut tree = value_from_owned(value).map_err(Error::Serialization)?;
        rewrite_bytes_to_array(&mut tree);
        serde_json::to_vec(&tree).map_err(|error| Error::Serialization(error.to_string()))
    }

    fn decode<T: Typwire>(bytes: &[u8]) -> Result<T, Error> {
        let mut tree: Value = serde_json::from_slice(bytes)
            .map_err(|error| Error::Deserialization(error.to_string()))?;
        rewrite_array_to_bytes(&mut tree);
        value_to_owned(tree).map_err(Error::Deserialization)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{Any, Bytes};

    #[test]
    fn primitive_round_trips() {
        let bytes = Json::encode(&42_i64).unwrap();
        assert_eq!(bytes, b"42");
        assert_eq!(Json::decode::<i64>(&bytes).unwrap(), 42);
    }

    #[test]
    fn bytes_round_trip_as_a_tagged_array_not_a_bare_json_array() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Struct {
            value: Bytes,
        }
        impl Typwire for Struct {}

        let original = Struct {
            value: Bytes::new(vec![1, 2, 3]),
        };
        let encoded = Json::encode(&original).unwrap();
        let text = String::from_utf8(encoded.clone()).unwrap();
        assert!(text.contains("\"typwire-type\":\"bytes\""), "got: {text}");
        assert_eq!(Json::decode::<Struct>(&encoded).unwrap(), original);
    }

    #[test]
    fn any_preserves_integer_vs_float() {
        let bytes = Json::encode(&5_i64).unwrap();
        assert!(matches!(
            Json::decode::<Any>(&bytes).unwrap(),
            Any::Integer(5)
        ));

        let bytes = Json::encode(&5.0_f64).unwrap();
        assert!(matches!(Json::decode::<Any>(&bytes).unwrap(), Any::Float(f) if f == 5.0));
    }

    #[test]
    fn any_holding_bytes_round_trips() {
        let bytes = Json::encode(&Bytes::new(vec![9, 8, 7])).unwrap();
        let Any::Bytes(decoded) = Json::decode::<Any>(&bytes).unwrap() else {
            panic!("expected Any::Bytes");
        };
        assert_eq!(decoded.as_slice(), &[9, 8, 7]);
    }
}
