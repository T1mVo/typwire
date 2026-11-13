use serde::{Deserialize, Serialize};

use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "alignment";

/// A horizontal alignment: `start`, `left`, `center`, `right` or `end`.
///
/// See the [Typst documentation](https://typst.app/docs/reference/layout/alignment/).
///
/// # Examples
///
/// ```
/// use typwire::{HAlignment, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn is_centered(alignment: HAlignment) -> Result<bool, String> {
///     Ok(alignment == HAlignment::Center)
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HAlignment {
    /// The start of the text direction.
    Start,
    /// The left edge.
    Left,
    /// The horizontal center.
    Center,
    /// The right edge.
    Right,
    /// The end of the text direction.
    End,
}

/// A vertical alignment: `top`, `horizon` or `bottom`.
///
/// See the [Typst documentation](https://typst.app/docs/reference/layout/alignment/).
///
/// # Examples
///
/// ```
/// use typwire::{VAlignment, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn is_top(alignment: VAlignment) -> Result<bool, String> {
///     Ok(alignment == VAlignment::Top)
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VAlignment {
    /// The top edge.
    Top,
    /// The vertical center.
    Horizon,
    /// The bottom edge.
    Bottom,
}

/// Where to align something along an axis: horizontally, vertically, or both,
/// like `center + horizon`.
///
/// At least one of `x`/`y` is always present, enforced on decode.
///
/// See the [Typst documentation](https://typst.app/docs/reference/layout/alignment/).
///
/// # Examples
///
/// ```
/// use typwire::{Alignment, HAlignment, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn is_centered(alignment: Alignment) -> Result<bool, String> {
///     Ok(alignment.x == Some(HAlignment::Center))
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(try_from = "AlignmentTypst", into = "AlignmentTypst")]
pub struct Alignment {
    /// The horizontal component, if any.
    pub x: Option<HAlignment>,
    /// The vertical component, if any.
    pub y: Option<VAlignment>,
}

impl Alignment {
    /// Creates a new `Alignment` from its horizontal and vertical components.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::{Alignment, HAlignment};
    ///
    /// let value = Alignment::new(Some(HAlignment::Left), None);
    /// assert_eq!(value.x, Some(HAlignment::Left));
    /// ```
    pub const fn new(x: Option<HAlignment>, y: Option<VAlignment>) -> Self {
        Self { x, y }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct AlignmentTypst {
    typwire_type: Tag<Alignment>,
    x: Option<HAlignment>,
    y: Option<VAlignment>,
}

impl Tagged for Alignment {
    const TYPE_NAME: &'static str = TYPE_NAME;
}

// Serialize into AlignmentTypst
impl From<Alignment> for AlignmentTypst {
    fn from(value: Alignment) -> Self {
        Self {
            typwire_type: Tag::new(),
            x: value.x,
            y: value.y,
        }
    }
}

// Deserialize from AlignmentTypst
impl TryFrom<AlignmentTypst> for Alignment {
    type Error = String;

    fn try_from(value: AlignmentTypst) -> Result<Self, Self::Error> {
        if value.x.is_none() && value.y.is_none() {
            return Err("Alignment must have at least one of x or y set".to_owned());
        }

        Ok(Self::new(value.x, value.y))
    }
}
