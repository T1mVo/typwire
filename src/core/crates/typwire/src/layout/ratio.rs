use serde::{Deserialize, Serialize};

use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "ratio";

/// A ratio of a whole, written in Typst as a number followed by a percent sign.
///
/// Stored as a fraction: `0.5` is `50%`.
///
/// See the [Typst documentation](https://typst.app/docs/reference/layout/ratio/).
///
/// # Examples
///
/// ```
/// use typwire::{Ratio, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn percent(ratio: Ratio) -> Result<String, String> {
///     Ok(ratio.to_percentage())
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[serde(from = "RatioTypst", into = "RatioTypst")]
pub struct Ratio {
    /// The ratio as a fraction, where `1.0` is 100%.
    pub ratio: f64,
}

impl Ratio {
    /// Creates a new `Ratio` instance from a given floating-point value.
    ///
    /// # Arguments
    ///
    /// * `ratio` - A floating-point value representing the ratio (expected to be between 0 and 1).
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Ratio;
    ///
    /// let ratio = Ratio::new(0.5);
    /// assert_eq!(ratio.ratio, 0.5);
    /// ```
    pub const fn new(ratio: f64) -> Self {
        Self { ratio }
    }

    /// Converts the ratio to a percentage string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Ratio;
    ///
    /// let ratio = Ratio::new(0.75);
    /// assert_eq!(ratio.to_percentage(), "75%");
    /// ```
    pub fn to_percentage(&self) -> String {
        format!("{}%", self.ratio * 100.0)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct RatioTypst {
    typwire_type: Tag<Ratio>,
    ratio: f64,
}

impl Tagged for Ratio {
    const TYPE_NAME: &'static str = TYPE_NAME;
}

// Serialize into RatioTypst
impl From<Ratio> for RatioTypst {
    fn from(value: Ratio) -> Self {
        RatioTypst {
            typwire_type: Tag::new(),
            ratio: value.ratio,
        }
    }
}

// Deserialize from RatioTypst
impl From<RatioTypst> for Ratio {
    fn from(value: RatioTypst) -> Self {
        Self::new(value.ratio)
    }
}

/// `#[serde(with = "crate::layout::fraction")]`: a `Ratio` as a bare
/// fraction (`0.5` for 50%), with no `typwire-type` dictionary around it.
///
/// For a field whose enclosing type's own tag already says it's a ratio --
/// every component of a color, a gradient stop's offset, ... A tagged
/// dictionary there would only cost wire bytes and a tag check per field.
pub(crate) mod fraction {
    use serde::{Deserialize, Deserializer, Serializer};

    use super::Ratio;

    pub(crate) fn serialize<S: Serializer>(
        ratio: &Ratio,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_f64(ratio.ratio)
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Ratio, D::Error> {
        f64::deserialize(deserializer).map(Ratio::new)
    }
}

impl Default for Ratio {
    fn default() -> Self {
        Ratio::new(1.0)
    }
}
