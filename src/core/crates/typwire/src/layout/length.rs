use serde::{Deserialize, Serialize};

use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "length";

/// A size or distance.
///
/// Stored in points.
///
/// See the [Typst documentation](https://typst.app/docs/reference/layout/length/).
///
/// # Examples
///
/// ```
/// use typwire::{Length, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn to_mm(length: Length) -> Result<f64, String> {
///     Ok(length.mm())
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(from = "LengthTypst", into = "LengthTypst")]
pub struct Length {
    points: f64,
}

impl Length {
    /// Creates a new `Length` instance from a given number of points.
    ///
    /// # Arguments
    ///
    /// * `points` - A floating-point value representing the length in points.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Length;
    ///
    /// let length = Length::new(72.0);
    /// assert_eq!(length.pt(), 72.0);
    /// ```
    pub const fn new(points: f64) -> Self {
        Self { points }
    }

    /// Returns the length in points.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Length;
    ///
    /// let length = Length::new(72.0);
    /// assert_eq!(length.pt(), 72.0);
    /// ```
    pub const fn pt(&self) -> f64 {
        self.points
    }

    /// Converts the length to millimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Length;
    ///
    /// let length = Length::new(72.0);
    /// assert_eq!(length.mm(), 25.4);
    /// ```
    pub const fn mm(&self) -> f64 {
        self.points * (25.4 / 72.0)
    }

    /// Converts the length to centimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Length;
    ///
    /// let length = Length::new(720.0);
    /// assert_eq!(length.cm(), 25.4);
    /// ```
    pub const fn cm(&self) -> f64 {
        self.points * (25.4 / 720.0)
    }

    /// Converts the length to inches.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Length;
    ///
    /// let length = Length::new(72.0);
    /// assert_eq!(length.inches(), 1.0);
    /// ```
    pub const fn inches(&self) -> f64 {
        self.points / 72.0
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct LengthTypst {
    typwire_type: Tag<Length>,
    points: f64,
}

impl Tagged for Length {
    const TYPE_NAME: &'static str = TYPE_NAME;
}

// Serialize into LengthTypst
impl From<Length> for LengthTypst {
    fn from(value: Length) -> Self {
        Self {
            typwire_type: Tag::new(),
            points: value.points,
        }
    }
}

// Deserialize from LengthTypst
impl From<LengthTypst> for Length {
    fn from(value: LengthTypst) -> Self {
        Length::new(value.points)
    }
}
