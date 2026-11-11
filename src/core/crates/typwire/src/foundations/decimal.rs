#![cfg(feature = "decimal")]

use serde::{Deserialize, Serialize};

use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "decimal";

/// A fixed-point decimal number type for precise base-10 arithmetic, such as
/// currency.
///
/// Backed by [`rust_decimal::Decimal`], the same type Typst wraps, and derefs to
/// it. Only available with the `decimal` feature.
///
/// See the [Typst documentation](https://typst.app/docs/reference/foundations/decimal/).
///
/// # Examples
///
/// ```
/// use typwire::{Decimal, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn is_negative(value: Decimal) -> Result<bool, String> {
///     Ok(value.is_sign_negative())
/// }
/// ```
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
    typwire_type: Tag<Decimal>,
    decimal: String,
}

impl Tagged for Decimal {
    const TYPE_NAME: &'static str = TYPE_NAME;
}

// Serialize into DecimalTypst
impl From<Decimal> for DecimalTypst {
    fn from(value: Decimal) -> Self {
        Self {
            typwire_type: Tag::new(),
            decimal: value.0.to_string(),
        }
    }
}

// Deserialize from DecimalTypst
impl TryFrom<DecimalTypst> for Decimal {
    type Error = String;

    fn try_from(value: DecimalTypst) -> Result<Self, Self::Error> {
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
                typwire_type: Tag::new(),
                decimal: "not-a-number".to_string(),
            },
        )
        .unwrap();

        assert!(decode::<Decimal>(&bytes).is_err());
    }
}
