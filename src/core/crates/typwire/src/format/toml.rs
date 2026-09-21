use cbor4ii::core::Value;

use crate::Typwire;
use crate::wire::{
    Error, Format, rewrite_array_to_bytes, rewrite_bytes_to_array, value_from_owned, value_to_owned,
};

/// The `toml` wire format, backed by the `toml` crate.
///
/// Same `Bytes` handling as [`crate::Json`] -- see its doc comment -- since
/// TOML has no native byte-string type either. Unlike JSON/YAML, TOML's own
/// grammar requires a top-level table: encoding a `Typwire` value that
/// isn't struct/dictionary-shaped (a bare `i64`, a `Vec<T>`, ...) at the
/// root fails with a clear error from the `toml` crate itself, the same way
/// Typst's own `toml.encode` already rejects a non-dictionary top-level
/// value.
pub struct Toml;

impl Format for Toml {
    fn encode<T: Typwire>(value: &T) -> Result<Vec<u8>, Error> {
        let mut tree = value_from_owned(value).map_err(Error::Serialization)?;
        rewrite_bytes_to_array(&mut tree);
        toml::to_string(&tree)
            .map(String::into_bytes)
            .map_err(|error| Error::Serialization(error.to_string()))
    }

    fn decode<T: Typwire>(bytes: &[u8]) -> Result<T, Error> {
        let text = std::str::from_utf8(bytes)
            .map_err(|error| Error::Deserialization(error.to_string()))?;
        let mut tree: Value =
            toml::from_str(text).map_err(|error| Error::Deserialization(error.to_string()))?;
        rewrite_array_to_bytes(&mut tree);
        value_to_owned(tree).map_err(Error::Deserialization)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{Any, Bytes};

    #[test]
    fn bare_scalar_at_the_top_level_is_rejected() {
        // TOML's own grammar requires a top-level table -- matches Typst's
        // own `toml.encode`, confirmed empirically to reject `toml.encode(5)`
        // the same way.
        assert!(Toml::encode(&42_i64).is_err());
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
