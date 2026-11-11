use serde::{Deserialize, Serialize};

use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "angle";

/// An angle describing a rotation.
///
/// Stored in radians.
///
/// See the [Typst documentation](https://typst.app/docs/reference/layout/angle/).
///
/// # Examples
///
/// ```
/// use typwire::{Angle, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn degrees(angle: Angle) -> Result<f64, String> {
///     Ok(angle.deg())
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
#[serde(from = "AngleTypst", into = "AngleTypst")]
pub struct Angle {
    radians: f64,
}

impl Angle {
    /// Creates a new `Angle` instance from the given radians.
    ///
    /// # Arguments
    ///
    /// * `radians` - The angle in radians to create an `Angle` from.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Angle;
    ///
    /// let angle = Angle::new(1.0);
    /// assert_eq!(angle.rad(), 1.0);
    /// ```
    pub const fn new(radians: f64) -> Self {
        Self { radians }
    }

    /// Returns the angle in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Angle;
    ///
    /// let angle = Angle::new(1.0);
    /// assert_eq!(angle.rad(), 1.0);
    /// ```
    pub const fn rad(&self) -> f64 {
        self.radians
    }

    /// Returns the angle in degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Angle;
    ///
    /// let angle = Angle::new(std::f64::consts::PI);
    /// assert_eq!(angle.deg(), 180.0);
    /// ```
    pub const fn deg(&self) -> f64 {
        self.radians * 180.0 / std::f64::consts::PI
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct AngleTypst {
    typwire_type: Tag<Angle>,
    radians: f64,
}

impl Tagged for Angle {
    const TYPE_NAME: &'static str = TYPE_NAME;
}

// Serialize into AngleTypst
impl From<Angle> for AngleTypst {
    fn from(angle: Angle) -> Self {
        AngleTypst {
            typwire_type: Tag::new(),
            radians: angle.radians,
        }
    }
}

// Deserialize from AngleTypst
impl From<AngleTypst> for Angle {
    fn from(value: AngleTypst) -> Self {
        Self::new(value.radians)
    }
}

/// `#[serde(with = "crate::layout::radians")]`: an `Angle` as a bare number
/// of radians, with no `typwire-type` dictionary around it -- the angle
/// counterpart of `crate::layout::fraction`.
pub(crate) mod radians {
    use serde::{Deserialize, Deserializer, Serializer};

    use super::Angle;

    pub(crate) fn serialize<S: Serializer>(
        angle: &Angle,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_f64(angle.radians)
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Angle, D::Error> {
        f64::deserialize(deserializer).map(Angle::new)
    }
}
