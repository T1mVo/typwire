use cbor4ii::core::Value;

use crate::Typwire;
use crate::wire::{
    Error, Format, rewrite_array_to_bytes, rewrite_bytes_to_array, value_from_owned, value_to_owned,
};

/// The `yaml` wire format, backed by `serde-saphyr`.
///
/// Same `Bytes` handling as [`crate::Json`] -- see its doc comment -- since
/// YAML has no native byte-string type either.
///
/// `decode` parses into `serde_json::Value` first, *not* directly into
/// `cbor4ii::core::Value` the way every other format does -- confirmed
/// empirically, this is a real, YAML-specific bug, not a stylistic choice:
/// `cbor4ii::core::Value`'s `Deserialize` reads map *keys* generically (it
/// supports non-string CBOR keys, so it can't just assume `String`), which
/// drives `serde-saphyr` through legacy YAML 1.1 core-schema scalar
/// resolution for that position -- the "Norway problem," where a bare `y`
/// key silently becomes the boolean `true` instead of the string `"y"`
/// (confirmed with a standalone probe: `{y: 1}` parses as `Map([(Bool(true),
/// Integer(1))])` via `cbor4ii::core::Value`, but correctly as
/// `Object {"y": 1}` via `serde_json::Value` -- whose map keys are always
/// typed `String`, sidestepping the ambiguous scalar resolution entirely).
/// Both `Alignment`/`Center` (literal `x`/`y` field names) hit this for
/// real. `serde_json::from_value` then converts the already-safely-parsed
/// tree into `cbor4ii::core::Value`, a second, purely in-memory conversion
/// with no text to misinterpret, so the rest of the pipeline (`Bytes`/
/// `none` rewriting, `value_to_owned`) is unchanged. `encode` is unaffected
/// -- the ambiguity is a parsing-time (text -> value) concern only.
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
        let json_value: serde_json::Value = serde_saphyr::from_str(text)
            .map_err(|error| Error::Deserialization(error.to_string()))?;
        let mut tree: Value = serde_json::from_value(json_value)
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
    fn bare_y_key_does_not_become_a_boolean() {
        // Regression test for the "Norway problem" documented on `Yaml`
        // above: a struct field literally named `y` (as `Center`/
        // `Alignment` both have) used to come back with a boolean `y` key
        // in the intermediate tree instead of the string key the derived
        // `Deserialize` was looking for.
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Point {
            x: i64,
            y: i64,
        }
        impl Typwire for Point {}

        let original = Point { x: 1, y: 2 };
        let bytes = Yaml::encode(&original).unwrap();
        assert_eq!(Yaml::decode::<Point>(&bytes).unwrap(), original);
    }

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
