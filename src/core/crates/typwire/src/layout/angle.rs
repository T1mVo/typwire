use serde::{Deserialize, Serialize};

const TYPE_NAME: &str = "angle";

/// A structure representing an angle in radians.
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
#[serde(try_from = "AngleCbor", into = "AngleCbor")]
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
struct AngleCbor {
    typwire_type: String,
    radians: f64,
}

// Serialize into AngleCbor
impl From<Angle> for AngleCbor {
    fn from(angle: Angle) -> Self {
        AngleCbor {
            typwire_type: TYPE_NAME.to_string(),
            radians: angle.radians,
        }
    }
}

// Deserialize from AngleCbor
impl TryFrom<AngleCbor> for Angle {
    type Error = String;

    fn try_from(value: AngleCbor) -> Result<Self, Self::Error> {
        if value.typwire_type != TYPE_NAME {
            return Err(format!(
                "Invalid typwire-type for Angle: {}",
                value.typwire_type
            ));
        }

        Ok(Self::new(value.radians))
    }
}
