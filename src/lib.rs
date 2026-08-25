pub use angle::Angle;
pub use any::Any;
pub use center::Center;
pub use color::{Cmyk, Color, ColorGradient, Hsl, Hsv, LinearRgb, Luma, Oklab, Oklch, Rgb};
pub use datetime::{DateTime, DateTimeBuilder};
pub use duration::{Duration, DurationBuilder};
pub use gradient::{
    ColorSpace, ConicGradientBuilder, Gradient, LinearGradientBuilder, RadialGradientBuilder,
};
pub use length::{Length, LengthRadius};
pub use one_of::{OneOf2, OneOf3, OneOf4, OneOf5, OneOf6, OneOf7, OneOf8};
pub use radius::Radius;
pub use ratio::Ratio;
pub use stop::Stop;
pub use r#type::Type;
pub use typwire_macros::{Typwire, export};
pub use version::Version;
pub use wasm_minimal_protocol::initiate_protocol;
pub use wire::{Bytes, Error, Typwire, decode, encode};

mod angle;
mod any;
mod center;
pub mod color;
mod datetime;
mod duration;
mod gradient;
mod length;
mod one_of;
mod radius;
mod ratio;
mod stop;
mod r#type;
mod version;
mod wire;

macro_rules! impl_typwire {
    ($($ty:ty),+ $(,)?) => {
        $(impl Typwire for $ty {})+
    };
}

impl_typwire!(
    Angle,
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
    DateTime,
    Duration,
    Gradient,
    ColorSpace,
    Length,
    LengthRadius,
    Radius,
    Ratio,
    Stop,
    Type,
    Version,
);

#[doc(hidden)]
pub mod __private {
    use std::fmt::Display;

    use super::{Typwire, decode, encode};

    pub use serde::{Serialize, de::DeserializeOwned};
    pub use wasm_minimal_protocol::wasm_func;

    pub fn decode_argument<T: Typwire>(
        bytes: &[u8],
        function: &str,
        argument: &str,
    ) -> Result<T, String> {
        decode(bytes).map_err(|error| {
            format!("failed to decode argument `{argument}` for `{function}`: {error}")
        })
    }

    pub fn encode_result<T: Typwire>(value: &T, function: &str) -> Result<Vec<u8>, String> {
        encode(value).map_err(|error| format!("failed to encode result of `{function}`: {error}"))
    }

    pub fn display_error(error: impl Display) -> String {
        error.to_string()
    }
}

extern crate self as typwire;
