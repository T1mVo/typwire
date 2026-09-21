//! Types from Typst's `foundations` category: <https://typst.app/docs/reference/foundations/>.

mod arguments;
mod auto;
mod datetime;
#[cfg(feature = "decimal")]
mod decimal;
mod duration;
mod path;
#[cfg(feature = "regex")]
mod regex;
mod symbol;
mod r#type;
mod version;

pub use arguments::Arguments;
pub use auto::Auto;
pub use datetime::{DateTime, DateTimeBuilder};
#[cfg(feature = "decimal")]
pub use decimal::Decimal;
pub use duration::{Duration, DurationBuilder};
pub use path::Path;
#[cfg(feature = "regex")]
pub use regex::Regex;
pub use symbol::{Symbol, SymbolVariant};
pub use r#type::Type;
pub use version::Version;
