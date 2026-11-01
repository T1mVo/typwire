use serde::{Deserialize, Serialize};

use crate::tag::{Tag, Tagged};

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
#[serde(from = "AutoTypst", into = "AutoTypst")]
pub struct Auto;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct AutoTypst {
    typwire_type: Tag<Auto>,
}

impl Tagged for Auto {
    const TYPE_NAME: &'static str = TYPE_NAME;
}

// Serialize into AutoTypst
impl From<Auto> for AutoTypst {
    fn from(_: Auto) -> Self {
        Self {
            typwire_type: Tag::new(),
        }
    }
}

// Deserialize from AutoTypst
impl From<AutoTypst> for Auto {
    fn from(_: AutoTypst) -> Self {
        Auto
    }
}
