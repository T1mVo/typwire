use std::borrow::Cow;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::tag::{Tag, Tagged};
use crate::{Auto, Color, Gradient, Length, OneOf2};

const TYPE_NAME: &str = "stroke";

/// The paint used by a stroke: a solid color or a gradient.
///
/// Typst also allows a tiling pattern as a stroke's paint; that variant has
/// no `Typwire` representation (no `Pattern`/`Tiling` type exists in this
/// crate yet), so encoding a pattern-painted stroke fails the same way any
/// other unsupported value does.
pub type Paint = OneOf2<Color, Gradient>;

/// How the ends of a stroke are rendered: `butt`, `round` or `square`.
///
/// See the [Typst documentation](https://typst.app/docs/reference/visualize/stroke/).
///
/// # Examples
///
/// ```
/// use typwire::{LineCap, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn is_round(cap: LineCap) -> Result<bool, String> {
///     Ok(cap == LineCap::Round)
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

/// How sharp turns in a stroke are rendered: `miter`, `round` or `bevel`.
///
/// See the [Typst documentation](https://typst.app/docs/reference/visualize/stroke/).
///
/// # Examples
///
/// ```
/// use typwire::{LineJoin, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn is_round(join: LineJoin) -> Result<bool, String> {
///     Ok(join == LineJoin::Round)
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

/// One entry of a stroke's dash array: a length, or `"dot"` (a dash as long as
/// the stroke is thick).
///
/// Carries no `typwire-type` tag: Typst represents a `.dash` field as a plain
/// dictionary, so this nests inside [`Stroke`] via ordinary Serde.
///
/// See the [Typst documentation](https://typst.app/docs/reference/visualize/stroke/).
///
/// # Examples
///
/// ```
/// use typwire::{DashLength, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn is_dot(entry: DashLength) -> Result<bool, String> {
///     Ok(matches!(entry, DashLength::Dot))
/// }
/// ```
#[derive(Clone, Debug)]
pub enum DashLength {
    Length(Length),
    Dot,
}

impl Serialize for DashLength {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Length(value) => value.serialize(serializer),
            Self::Dot => serializer.serialize_str("dot"),
        }
    }
}

impl<'de> Deserialize<'de> for DashLength {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Repr {
            Length(Length),
            Marker(String),
        }

        match Repr::deserialize(deserializer)? {
            Repr::Length(value) => Ok(Self::Length(value)),
            Repr::Marker(text) if text == "dot" => Ok(Self::Dot),
            Repr::Marker(text) => Err(serde::de::Error::custom(format!(
                "expected a length or the dash marker \"dot\", found {text:?}"
            ))),
        }
    }
}

/// A stroke's dash pattern: alternating dash and gap lengths plus a phase
/// offset.
///
/// Untagged, like [`DashLength`].
///
/// See the [Typst documentation](https://typst.app/docs/reference/visualize/stroke/).
///
/// # Examples
///
/// ```
/// use typwire::{DashPattern, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn segments(pattern: DashPattern) -> Result<i64, String> {
///     Ok(pattern.array.len() as i64)
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DashPattern {
    pub array: Vec<DashLength>,
    pub phase: Length,
}

/// Defines how to draw a line: a paint, thickness, line cap, line join, miter
/// limit and dash pattern.
///
/// Every field may be `auto`, modeled as `OneOf2<Auto, _>`.
///
/// See the [Typst documentation](https://typst.app/docs/reference/visualize/stroke/).
///
/// # Examples
///
/// ```
/// use typwire::{Length, OneOf2, Stroke, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn thickness(stroke: Stroke) -> Result<Option<Length>, String> {
///     Ok(match stroke.thickness {
///         OneOf2::First(_) => None,
///         OneOf2::Second(length) => Some(length),
///     })
/// }
/// ```
#[derive(Deserialize, Clone, Debug)]
#[serde(from = "StrokeTypst<'static>")]
pub struct Stroke {
    pub paint: OneOf2<Auto, Paint>,
    pub thickness: OneOf2<Auto, Length>,
    pub cap: OneOf2<Auto, LineCap>,
    pub join: OneOf2<Auto, LineJoin>,
    pub dash: OneOf2<Auto, Option<DashPattern>>,
    pub miter_limit: OneOf2<Auto, f64>,
}

impl Stroke {
    /// Creates a new `Stroke` from its fields.
    pub const fn new(
        paint: OneOf2<Auto, Paint>,
        thickness: OneOf2<Auto, Length>,
        cap: OneOf2<Auto, LineCap>,
        join: OneOf2<Auto, LineJoin>,
        dash: OneOf2<Auto, Option<DashPattern>>,
        miter_limit: OneOf2<Auto, f64>,
    ) -> Self {
        Self {
            paint,
            thickness,
            cap,
            join,
            dash,
            miter_limit,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct StrokeTypst<'a> {
    typwire_type: Tag<Stroke>,
    paint: Cow<'a, OneOf2<Auto, Paint>>,
    thickness: OneOf2<Auto, Length>,
    cap: OneOf2<Auto, LineCap>,
    join: OneOf2<Auto, LineJoin>,
    dash: Cow<'a, OneOf2<Auto, Option<DashPattern>>>,
    miter_limit: OneOf2<Auto, f64>,
}

impl Tagged for Stroke {
    const TYPE_NAME: &'static str = TYPE_NAME;
}

// Serialize through a borrowing StrokeTypst -- `#[serde(into = ...)]`
// would clone the whole value first.
impl Serialize for Stroke {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        StrokeTypst::from(self).serialize(serializer)
    }
}

impl<'a> From<&'a Stroke> for StrokeTypst<'a> {
    fn from(value: &'a Stroke) -> Self {
        Self {
            typwire_type: Tag::new(),
            paint: Cow::Borrowed(&value.paint),
            thickness: value.thickness.clone(),
            cap: value.cap.clone(),
            join: value.join.clone(),
            dash: Cow::Borrowed(&value.dash),
            miter_limit: value.miter_limit.clone(),
        }
    }
}

// Deserialize from StrokeTypst
impl From<StrokeTypst<'_>> for Stroke {
    fn from(value: StrokeTypst<'_>) -> Self {
        Self::new(
            value.paint.into_owned(),
            value.thickness,
            value.cap,
            value.join,
            value.dash.into_owned(),
            value.miter_limit,
        )
    }
}
