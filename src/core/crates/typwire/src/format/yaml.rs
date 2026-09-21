use cbor4ii::core::Value;

use crate::Typwire;
use crate::wire::{
    Error, Format, rewrite_array_to_bytes, rewrite_bytes_to_array, value_from_owned, value_to_owned,
};

/// The `yaml` wire format, backed by `serde-saphyr`.
///
/// Same `Bytes` handling as [`crate::Json`] -- see its doc comment -- since
/// YAML has no native byte-string type either.
pub struct Yaml;

impl Format for Yaml {
    fn encode<T: Typwire>(value: &T) -> Result<Vec<u8>, Error> {
        let mut tree = value_from_owned(value).map_err(Error::Serialization)?;
        rewrite_bytes_to_array(&mut tree);
        serde_saphyr::to_string(&tree)
            .map(String::into_bytes)
            .map_err(|error| Error::Serialization(error.to_string()))
    }

    fn decode<T: Typwire>(bytes: &[u8]) -> Result<T, Error> {
        let text = std::str::from_utf8(bytes)
            .map_err(|error| Error::Deserialization(error.to_string()))?;
        let mut tree: Value = serde_saphyr::from_str(text)
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
        let bytes = Yaml::encode(&42_i64).unwrap();
        assert_eq!(Yaml::decode::<i64>(&bytes).unwrap(), 42);
    }

    #[test]
    fn bytes_round_trip_as_a_tagged_array() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Struct {
            value: Bytes,
        }
        impl Typwire for Struct {}

        let original = Struct {
            value: Bytes::new(vec![1, 2, 3]),
        };
        let encoded = Yaml::encode(&original).unwrap();
        assert_eq!(Yaml::decode::<Struct>(&encoded).unwrap(), original);
    }

    #[test]
    fn any_preserves_integer_vs_float() {
        let bytes = Yaml::encode(&5_i64).unwrap();
        assert!(matches!(
            Yaml::decode::<Any>(&bytes).unwrap(),
            Any::Integer(5)
        ));

        let bytes = Yaml::encode(&5.0_f64).unwrap();
        assert!(matches!(Yaml::decode::<Any>(&bytes).unwrap(), Any::Float(f) if f == 5.0));
    }

    #[test]
    fn any_holding_bytes_round_trips() {
        let bytes = Yaml::encode(&Bytes::new(vec![9, 8, 7])).unwrap();
        let Any::Bytes(decoded) = Yaml::decode::<Any>(&bytes).unwrap() else {
            panic!("expected Any::Bytes");
        };
        assert_eq!(decoded.as_slice(), &[9, 8, 7]);
    }
}
