use cbor4ii::core::Value;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::Typwire;
use crate::wire::{Error, Format};

/// The `json` wire format, backed by `serde_json`.
///
/// `Bytes` has no native JSON representation, so `encode`/`decode` round-trip
/// the value through a generic [`Value`] tree first and rewrite any
/// [`Value::Bytes`] node into the same tagged array shape the Typst side
/// uses (`src/wire/types/foundations/bytes.typ`) -- `Cbor` needs none of
/// this, since CBOR has a native byte-string type. `value_from_owned`/
/// `value_to_owned`/`rewrite_bytes_to_array`/`rewrite_array_to_bytes` below
/// used to live in `wire.rs`, shared by three formats (JSON, YAML, TOML)
/// back when this crate supported all of them, and by `Any`, back when it
/// decoded through a `Value` tree too; `Json` is the only caller left.
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

/// Turn any `Typwire` value into a generic [`Value`] tree directly, with no
/// CBOR bytes in between -- see `format/value.rs::to_value`. Gets a tree
/// [`encode`](Format::encode) can rewrite [`crate::Bytes`] nodes in before
/// handing it to `serde_json`, which has no native byte-string concept --
/// see [`rewrite_bytes_to_array`].
fn value_from_owned<T: Serialize>(value: &T) -> Result<Value, String> {
    crate::format::to_value(value).map_err(|error| error.to_string())
}

/// The mirror of [`value_from_owned`]: reconstruct any `Typwire` value
/// directly from a generic [`Value`] tree -- see
/// `format/value.rs::from_value`.
fn value_to_owned<T: DeserializeOwned>(value: Value) -> Result<T, String> {
    crate::format::from_value(value).map_err(|error| error.to_string())
}

/// Replace every [`Value::Bytes`] node in `value` with the same tagged
/// dictionary shape the Typst side uses for a format with no native
/// byte-string type (`src/wire/types/foundations/bytes.typ`): `{typwire-type:
/// "bytes", data: [...]}`, `data` a plain array of byte integers -- not
/// base64, since Typst has no built-in base64 codec to match it against.
fn rewrite_bytes_to_array(value: &mut Value) {
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
fn rewrite_array_to_bytes(value: &mut Value) {
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
