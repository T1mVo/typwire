//! Types from Typst's `foundations` category: <https://typst.app/docs/reference/foundations/>.

mod arguments;
mod array;
mod auto;
mod bool;
mod datetime;
#[cfg(feature = "decimal")]
mod decimal;
mod dictionary;
mod duration;
mod float;
mod int;
mod label;
mod path;
#[cfg(feature = "regex")]
mod regex;
mod str;
mod symbol;
mod r#type;
mod version;

pub use arguments::Arguments;
pub use array::Array;
pub use auto::Auto;
pub use bool::Bool;
pub use datetime::{DateTime, DateTimeBuilder};
#[cfg(feature = "decimal")]
pub use decimal::Decimal;
pub use dictionary::Dictionary;
pub use duration::{Duration, DurationBuilder};
pub use float::Float;
pub use int::Int;
pub use label::Label;
pub use path::Path;
#[cfg(feature = "regex")]
pub use regex::Regex;
pub use str::Str;
pub use symbol::{Symbol, SymbolVariant};
pub use r#type::Type;
pub use version::Version;
