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
#[serde(try_from = "AutoTypst", into = "AutoTypst")]
pub struct Auto;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct AutoTypst {
    typwire_type: String,
}

// Serialize into AutoTypst
impl From<Auto> for AutoTypst {
    fn from(_: Auto) -> Self {
        Self {
            typwire_type: TYPE_NAME.to_string(),
        }
    }
}

// Deserialize from AutoTypst
impl TryFrom<AutoTypst> for Auto {
    type Error = String;

    fn try_from(value: AutoTypst) -> Result<Self, Self::Error> {
        if value.typwire_type != TYPE_NAME {
            return Err(format!(
                "Invalid typwire-type for Auto: {}",
                value.typwire_type
            ));
        }

        Ok(Auto)
    }
}
