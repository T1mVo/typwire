use serde::{Deserialize, Serialize};

const TYPE_NAME: &str = "auto";

/// Typst's `auto`: a value that indicates a smart default.
///
/// Distinct from [`Option::None`], which represents the absence of a value;
/// `auto` instead tells the receiver to pick its own default.
///
/// # Examples
///
/// ```
/// use typwire::Auto;
///
/// let value = Auto;
/// ```
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(try_from = "AutoCbor", into = "AutoCbor")]
pub struct Auto;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct AutoCbor {
    typwire_type: String,
}

// Serialize into AutoCbor
impl From<Auto> for AutoCbor {
    fn from(_: Auto) -> Self {
        Self {
            typwire_type: TYPE_NAME.to_string(),
        }
    }
}

// Deserialize from AutoCbor
impl TryFrom<AutoCbor> for Auto {
    type Error = String;

    fn try_from(value: AutoCbor) -> Result<Self, Self::Error> {
        if value.typwire_type != TYPE_NAME {
            return Err(format!(
                "Invalid typwire-type for Auto: {}",
                value.typwire_type
            ));
        }

        Ok(Auto)
    }
}
