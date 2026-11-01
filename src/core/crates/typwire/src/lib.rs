pub use any::Any;
pub use format::Cbor;
#[cfg(feature = "json")]
pub use format::Json;
#[cfg(feature = "decimal")]
pub use foundations::Decimal;
#[cfg(feature = "regex")]
pub use foundations::Regex;
pub use foundations::{
    Arguments, Array, Auto, Bool, DateTime, DateTimeBuilder, Dictionary, Duration, DurationBuilder,
    Float, Int, Label, Path, Str, Symbol, SymbolVariant, Type, Version,
};
pub use layout::{Alignment, Angle, HAlignment, Length, Radius, Ratio, VAlignment};
pub use one_of::{OneOf2, OneOf3, OneOf4, OneOf5, OneOf6, OneOf7, OneOf8};
pub use typwire_macros::{Typwire, export};
pub use visualize::{
    Center, Cmyk, Color, ColorGradient, ColorSpace, ConicGradientBuilder, DashLength, DashPattern,
    Gradient, Hsl, Hsv, LineCap, LineJoin, LinearGradientBuilder, LinearRgb, Luma, Oklab, Oklch,
    Paint, RadialGradientBuilder, Rgb, Stop, Stroke, color,
};
pub use wasm_minimal_protocol::initiate_protocol;
pub use wire::{Bytes, Error, Format, Typwire, decode, encode};

mod any;
mod format;
mod foundations;
mod layout;
mod one_of;
mod tag;
mod visualize;
mod wire;

macro_rules! impl_typwire {
    ($($ty:ty),+ $(,)?) => {
        $(impl Typwire for $ty {})+
    };
}

impl_typwire!(
    Alignment,
    Angle,
    Arguments,
    Auto,
    Bool,
    Float,
    Int,
    Str,
    Center,
    Luma,
    Oklab,
    Oklch,
    LinearRgb,
    Rgb,
    Cmyk,
    Hsl,
    Hsv,
    Color,
    ColorGradient,
    DashLength,
    DashPattern,
    DateTime,
    Duration,
    Gradient,
    ColorSpace,
    HAlignment,
    Label,
    Length,
    LineCap,
    LineJoin,
    Path,
    Radius,
    Ratio,
    Stop,
    Stroke,
    Symbol,
    Type,
    VAlignment,
    Version,
);

#[cfg(feature = "decimal")]
impl Typwire for Decimal {}

#[cfg(feature = "regex")]
impl Typwire for Regex {}

#[doc(hidden)]
pub mod __private {
    use std::fmt::Display;

    use super::{Format, Typwire};

    pub use serde::{Serialize, de::DeserializeOwned};
    pub use wasm_minimal_protocol::wasm_func;

    pub fn decode_argument<F: Format, T: Typwire>(
        bytes: &[u8],
        function: &str,
        argument: &str,
    ) -> Result<T, String> {
        F::decode(bytes).map_err(|error| {
            format!("failed to decode argument `{argument}` for `{function}`: {error}")
        })
    }

    pub fn encode_result<F: Format, T: Typwire>(
        value: &T,
        function: &str,
    ) -> Result<Vec<u8>, String> {
        F::encode(value)
            .map_err(|error| format!("failed to encode result of `{function}`: {error}"))
    }

    pub fn display_error(error: impl Display) -> String {
        error.to_string()
    }
}

extern crate self as typwire;
