use serde::{Deserialize, Serialize};

use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "alignment";

/// Where to align something horizontally.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HAlignment {
    Start,
    Left,
    Center,
    Right,
    End,
}

/// Where to align something vertically.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VAlignment {
    Top,
    Horizon,
    Bottom,
}

/// A structure representing a Typst 1D or 2D alignment.
///
/// Mirrors Typst's own `Alignment::H`/`V`/`Both`: at least one of `x`/`y` is
/// always present, enforced on decode.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(try_from = "AlignmentTypst", into = "AlignmentTypst")]
pub struct Alignment {
    pub x: Option<HAlignment>,
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
