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

/// How the ends of a stroke are rendered.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

/// How sharp turns in a stroke are rendered.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

/// One entry of a stroke's dash array: either a length, or Typst's `"dot"`
/// marker (a dash the same length as the stroke's own thickness).
///
/// Not itself a `typwire-type`-tagged native value -- Typst represents a
/// concrete `.dash` field as a plain dictionary, not a distinct runtime
/// type (confirmed empirically: `type(stroke(dash: "dashed").dash)` is
/// `dictionary`, not some dedicated dash-pattern type) -- so this, like
/// [`DashPattern`], round-trips via ordinary Serde nesting inside
/// [`Stroke`]'s own tagged dictionary rather than carrying a tag of its own.
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

/// A stroke's dash pattern: alternating dash/gap lengths plus a phase
/// offset. See [`DashLength`] for why this carries no `typwire-type` tag.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DashPattern {
    pub array: Vec<DashLength>,
    pub phase: Length,
}

/// A structure representing a Typst line stroke.
///
/// Every field is `auto`-able, matching Typst's own `Stroke`: each is
/// wired to the wire's `Auto` value via `OneOf2<Auto, _>` rather than a
/// bespoke "smart" wrapper, the same composition `OneOf2`/`OneOf3` already
/// exist for.
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
