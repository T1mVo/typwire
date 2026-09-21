use cbor4ii::core::Value;

use crate::Typwire;
use crate::wire::{
    Error, Format, rewrite_array_to_bytes, rewrite_bytes_to_array, value_from_owned, value_to_owned,
};

/// The fixed envelope key every value is wrapped under -- see the doc
/// comment on [`Toml`] for why.
const ENVELOPE_KEY: &str = "value";

const NONE_TYPE_NAME: &str = "none";

/// The `toml` wire format, backed by the `toml` crate.
///
/// Same `Bytes` handling as [`crate::Json`] -- see its doc comment -- since
/// TOML has no native byte-string type either. TOML also has no `none`/null:
/// confirmed empirically, the `toml` crate hard-errors ("unsupported None
/// value") on one anywhere in the tree, so every `Value::Null` is rewritten
/// into the tagged dictionary `{typwire-type: "none"}` the same way `Bytes`
/// is, mirroring `src/wire/rewrite-none.typ` on the Typst side -- `Json`/
/// `Yaml`/`Cbor` represent `none`/null natively and don't need this.
///
/// TOML's own grammar also requires a top-level table, unlike JSON/YAML/
/// CBOR, which can encode any shape at the root -- so, unconditionally, the
/// whole tree is wrapped under one fixed key (`"value"`) before reaching
/// `toml::to_string`, the same way on both sides of the wire (see the
/// mirrored envelope in `typwire.toml`, `src/format/toml.typ`). Between the
/// envelope and the two rewrites, there is no longer any value this format
/// can't round-trip.
pub struct Toml;

impl Format for Toml {
    fn encode<T: Typwire>(value: &T) -> Result<Vec<u8>, Error> {
        let mut tree = value_from_owned(value).map_err(Error::Serialization)?;
        rewrite_bytes_to_array(&mut tree);
        rewrite_none_to_tagged(&mut tree);
        let envelope = Value::Map(vec![(Value::Text(ENVELOPE_KEY.to_owned()), tree)]);
        toml::to_string(&envelope)
            .map(String::into_bytes)
            .map_err(|error| Error::Serialization(error.to_string()))
    }

    fn decode<T: Typwire>(bytes: &[u8]) -> Result<T, Error> {
        let text = std::str::from_utf8(bytes)
            .map_err(|error| Error::Deserialization(error.to_string()))?;
        let envelope: Value =
            toml::from_str(text).map_err(|error| Error::Deserialization(error.to_string()))?;
        let Value::Map(entries) = envelope else {
            return Err(Error::Deserialization(
                "expected a TOML table wrapping the encoded value".to_owned(),
            ));
        };
        let mut tree = entries
            .into_iter()
            .find_map(|(key, value)| match key {
                Value::Text(key) if key == ENVELOPE_KEY => Some(value),
                _ => None,
            })
            .ok_or_else(|| {
                Error::Deserialization(format!("missing envelope key `{ENVELOPE_KEY}`"))
            })?;
        rewrite_array_to_bytes(&mut tree);
        rewrite_tagged_to_none(&mut tree);
        value_to_owned(tree).map_err(Error::Deserialization)
    }
}

/// Replace every [`Value::Null`] with `{typwire-type: "none"}` -- see the
/// doc comment on [`Toml`] for why.
fn rewrite_none_to_tagged(value: &mut Value) {
    match value {
        Value::Null => {
            *value = Value::Map(vec![(
                Value::Text("typwire-type".to_owned()),
                Value::Text(NONE_TYPE_NAME.to_owned()),
            )]);
        }
        Value::Array(items) => items.iter_mut().for_each(rewrite_none_to_tagged),
        Value::Map(entries) => entries
            .iter_mut()
            .for_each(|(_, entry)| rewrite_none_to_tagged(entry)),
        _ => {}
    }
}

/// The inverse of [`rewrite_none_to_tagged`].
fn rewrite_tagged_to_none(value: &mut Value) {
    match value {
        Value::Map(entries) => {
            let is_tagged_none = entries.iter().any(|(key, entry)| {
                matches!(
                    (key, entry),
                    (Value::Text(key), Value::Text(tag))
                        if key == "typwire-type" && tag == NONE_TYPE_NAME
                )
            });

            if is_tagged_none {
                *value = Value::Null;
            } else {
                entries
                    .iter_mut()
                    .for_each(|(_, entry)| rewrite_tagged_to_none(entry));
            }
        }
        Value::Array(items) => items.iter_mut().for_each(rewrite_tagged_to_none),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{Any, Bytes};

    #[test]
    fn bare_scalar_round_trips_via_the_envelope() {
        // TOML's own grammar requires a top-level table, so this only works
        // because of the envelope wrapper -- without it, this would be the
        // same "expected dictionary, found integer" failure Typst's own
        // `toml.encode(5)` hits.
        let bytes = Toml::encode(&42_i64).unwrap();
        assert_eq!(Toml::decode::<i64>(&bytes).unwrap(), 42);
    }

    #[test]
    fn none_round_trips_via_the_tagged_rewrite() {
        // `toml::to_string` hard-errors on a bare `None` -- confirmed
        // empirically -- so this only works because of the rewrite.
        let bytes = Toml::encode(&Option::<i64>::None).unwrap();
        assert_eq!(Toml::decode::<Option<i64>>(&bytes).unwrap(), None);

        let bytes = Toml::encode(&Some(5_i64)).unwrap();
        assert_eq!(Toml::decode::<Option<i64>>(&bytes).unwrap(), Some(5));
    }

    #[test]
    fn struct_round_trips() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Struct {
            value: Bytes,
            count: i64,
        }
        impl Typwire for Struct {}

        let original = Struct {
            value: Bytes::new(vec![1, 2, 3]),
            count: 5,
        };
        let encoded = Toml::encode(&original).unwrap();
        assert_eq!(Toml::decode::<Struct>(&encoded).unwrap(), original);
    }

    #[test]
    fn any_dictionary_preserves_integer_vs_float() {
        let mut map = BTreeMap::new();
        map.insert("i".to_owned(), Any::from(5_i64));
        map.insert("f".to_owned(), Any::from(5.0_f64));
        let value = Any::Dictionary(map);

        let bytes = Toml::encode(&value).unwrap();
        let Any::Dictionary(decoded) = Toml::decode::<Any>(&bytes).unwrap() else {
            panic!("expected Any::Dictionary");
        };
        assert!(matches!(decoded.get("i"), Some(Any::Integer(5))));
        assert!(matches!(decoded.get("f"), Some(Any::Float(f)) if *f == 5.0));
    }
}
