//! Types from Typst's `visualize` category: <https://typst.app/docs/reference/visualize/>.
//!
//! `color` is `pub` here (rather than a plain private `mod`) so that
//! `lib.rs` can re-export the whole module as `typwire::color`, preserving
//! its public path (e.g. `color::BLACK`) alongside the flat `Color` export.
//! This module itself stays private, so `typwire::visualize::color` is not
//! reachable from outside the crate -- only the `typwire::color` alias is.

mod center;
pub mod color;
mod gradient;
mod stop;
mod stroke;

pub use center::Center;
pub use color::{Cmyk, Color, ColorGradient, Hsl, Hsv, LinearRgb, Luma, Oklab, Oklch, Rgb};
pub use gradient::{
    ColorSpace, ConicGradientBuilder, Gradient, LinearGradientBuilder, RadialGradientBuilder,
};
pub use stop::Stop;
pub use stroke::{DashLength, DashPattern, LineCap, LineJoin, Paint, Stroke};
