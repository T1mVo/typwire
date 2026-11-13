//! Typed data exchange between a host and Rust WASM plugins.
//!
//! `typwire` turns an ordinary Rust function into a plugin export whose
//! arguments and return value are decoded and encoded for you. Every value
//! crosses the boundary as a typed, [Serde](https://serde.rs)-backed
//! message, so a plugin never handles raw bytes itself.
//!
//! # Installation
//!
//! ```sh
//! cargo add typwire
//! ```
//!
//! # Quick Start
//!
//! Call [`initiate_protocol!`] once at the crate root, then mark a function
//! with [`#[typwire::export]`](export). It takes decodable arguments and
//! returns a `Result`, whose error type only needs to implement `Display`:
//!
//! ```
//! use typwire::{Length, initiate_protocol};
//!
//! # use typwire::{Cbor, __private};
//! # fn main() {}
//! initiate_protocol!();
//!
//! #[typwire::export]
//! fn measure(label: String, scale: f64) -> Result<(String, Length), String> {
//!     Ok((format!("{label}{scale}"), Length::new(12.0 * scale)))
//! }
//! ```
//!
//! Arguments must be owned types: references are rejected at compile time.
//!
//! # Supported types
//!
//! - Primitives and collections: `bool`, `i64`, `f64`, `String`, `Option<T>`,
//!   `Vec<T>`, tuples and string-keyed maps, plus the named wrappers [`Int`],
//!   [`Float`], [`Str`], [`Bool`], [`Array`] and [`Dictionary`].
//! - Binary data: [`Bytes`]. A plain `Vec<u8>` is an array of integers
//!   instead.
//! - Built-in value types, such as [`Length`], [`Angle`], [`Ratio`],
//!   [`Color`], [`Gradient`], [`Stroke`], [`Alignment`], [`DateTime`],
//!   [`Duration`] and [`Version`]. `Decimal` and `Regex` are available
//!   behind their own features.
//! - [`Any`] for dynamic values, and [`OneOf2`] to [`OneOf8`] for a value
//!   that is one of several shapes, tried in declaration order.
//!
//! Deriving `Serialize` and `Deserialize` is not enough for a type of your
//! own to cross the boundary: it must also opt in with the [`Typwire`]
//! derive, so an unrelated Serde type never silently becomes part of a
//! plugin's interface.
//!
//! ```
//! use serde::{Deserialize, Serialize};
//! use typwire::{Length, Typwire, initiate_protocol};
//!
//! # use typwire::{Cbor, __private};
//! # fn main() {}
//! initiate_protocol!();
//!
//! #[derive(Serialize, Deserialize, Typwire)]
//! struct Size {
//!     width: Length,
//!     height: Length,
//! }
//!
//! #[typwire::export]
//! fn area(size: Size) -> Result<f64, String> {
//!     Ok(size.width.pt() * size.height.pt())
//! }
//! ```
//!
//! # Wire formats
//!
//! CBOR is the default format. Pick another per export with
//! `#[typwire::export(format = "json")]`, and give an export a different
//! name with `#[typwire::export(rename = "wire-name")]`; both options can be
//! combined. [`encode`] and [`decode`] always use CBOR, and each [`Format`]
//! can also be used directly, for example `Json::encode`.
//!
//! # Features
//!
//! - `json`: the `Json` wire format, backed by `serde_json`.
//! - `decimal`: the `Decimal` type, backed by `rust_decimal`.
//! - `regex`: the `Regex` type, backed by `regex-lite`.
//!
//! None are enabled by default. Naming a type or format whose feature is off
//! is a compile error.

pub use any::Any;
pub use format::Cbor;
#[cfg(feature = "json")]
pub use format::Json;
#[cfg(feature = "decimal")]
pub use foundations::Decimal;
#[cfg(feature = "regex")]
pub use foundations::Regex;
pub use foundations::{
    Arguments, Array, Auto, Bool, Bytes, DateTime, DateTimeBuilder, Dictionary, Duration,
    DurationBuilder, Float, Int, Label, Path, Str, Symbol, SymbolVariant, Type, Version,
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
pub use wire::{Error, Format, Typwire, decode, encode};

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
