#![cfg(feature = "decimal")]

use serde::{Deserialize, Serialize};

const TYPE_NAME: &str = "decimal";

/// A structure representing a Typst fixed-point decimal number.
///
/// Backed by [`rust_decimal::Decimal`] -- the same crate Typst's own
/// `decimal` type wraps internally (confirmed by reading
/// `typst-library/src/foundations/decimal.rs` upstream: `pub struct
/// Decimal(rust_decimal::Decimal);`), so this is exactly Typst's own
/// representation, not an approximation of it. `Decimal` derefs to
/// `rust_decimal::Decimal`, so its arithmetic methods are all directly
/// available.
///
/// This whole module -- the type, `Any::Decimal`, and every `Typwire`/
/// `From` impl for it -- only exists when this crate's `decimal` feature is
/// enabled. There's no placeholder type for the disabled case: a plugin
/// that references `Decimal` without the feature gets a compile error, not
/// a value that builds fine and only fails at wire-decode time. A
/// `decimal`-tagged value reaching `Any` without the feature falls through
/// to `Any`'s normal "unknown typwire-type tag" error instead of a
/// dedicated message -- see `any.rs`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "DecimalTypst", into = "DecimalTypst")]
pub struct Decimal(rust_decimal::Decimal);

impl Decimal {
    /// Parses a new `Decimal` from the given exact decimal string (e.g.
    /// `"3.14"`), the same textual form Typst's own `decimal(...)`
    /// constructor and `str()` conversion use.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Decimal;
    ///
    /// let d = Decimal::new("3.14").unwrap();
    /// assert_eq!(d.to_string(), "3.14");
    /// ```
    pub fn new(value: &str) -> Result<Self, rust_decimal::Error> {
        rust_decimal::Decimal::from_str_exact(value).map(Self)
    }
}

impl std::ops::Deref for Decimal {
    type Target = rust_decimal::Decimal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct DecimalTypst {
    typwire_type: String,
    decimal: String,
}

// Serialize into DecimalTypst
impl From<Decimal> for DecimalTypst {
    fn from(value: Decimal) -> Self {
        Self {
            typwire_type: TYPE_NAME.to_string(),
            decimal: value.0.to_string(),
        }
    }
}

// Deserialize from DecimalTypst
impl TryFrom<DecimalTypst> for Decimal {
    type Error = String;

    fn try_from(value: DecimalTypst) -> Result<Self, Self::Error> {
        if value.typwire_type != TYPE_NAME {
            return Err(format!(
                "Invalid typwire-type for Decimal: {}",
                value.typwire_type
            ));
        }

        Decimal::new(&value.decimal).map_err(|error| format!("invalid decimal: {error}"))
    }
}

#[cfg(test)]
mod tests {
    use crate::decode;

    use super::*;

    #[test]
    fn invalid_decimal_fails_to_decode() {
        // Bypasses `Decimal::new`'s own validation on purpose, to exercise
        // `TryFrom<DecimalTypst>`'s.
        let bytes = cbor4ii::serde::to_vec(
            Vec::new(),
            &DecimalTypst {
                typwire_type: TYPE_NAME.to_string(),
                decimal: "not-a-number".to_string(),
            },
        )
        .unwrap();

        assert!(decode::<Decimal>(&bytes).is_err());
    }
}
