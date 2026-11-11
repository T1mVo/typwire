use serde::{Deserialize, Serialize};

use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "auto";

/// A value that indicates a smart default. The type has exactly one value,
/// `auto`.
///
/// See the [Typst documentation](https://typst.app/docs/reference/foundations/auto/).
///
/// # Examples
///
/// ```
/// use typwire::{Auto, Length, OneOf2, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn describe(width: OneOf2<Auto, Length>) -> Result<String, String> {
///     Ok(match width {
///         OneOf2::First(Auto) => "auto".to_owned(),
///         OneOf2::Second(length) => format!("{}pt", length.pt()),
///     })
/// }
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
