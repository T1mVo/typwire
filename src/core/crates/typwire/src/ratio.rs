use serde::{Deserialize, Serialize};

const TYPE_NAME: &str = "ratio";

/// A structure representing a ratio from 0 to 1.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[serde(try_from = "RatioCbor", into = "RatioCbor")]
pub struct Ratio {
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
struct RatioCbor {
    typwire_type: String,
    ratio: f64,
}

// Serialize into RatioCbor
impl From<Ratio> for RatioCbor {
    fn from(value: Ratio) -> Self {
        RatioCbor {
            typwire_type: TYPE_NAME.to_string(),
            ratio: value.ratio,
        }
    }
}

// Deserialize from RatioCbor
impl TryFrom<RatioCbor> for Ratio {
    type Error = String;

    fn try_from(value: RatioCbor) -> Result<Self, Self::Error> {
        if value.typwire_type != TYPE_NAME {
            return Err(format!(
                "Invalid typwire-type for Ratio: {}",
                value.typwire_type
            ));
        }

        Ok(Self::new(value.ratio))
    }
}

impl Default for Ratio {
    fn default() -> Self {
        Ratio::new(1.0)
    }
}
