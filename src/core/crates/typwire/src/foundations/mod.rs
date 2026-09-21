//! Types from Typst's `foundations` category: <https://typst.app/docs/reference/foundations/>.

mod auto;
mod datetime;
mod duration;
mod path;
mod symbol;
mod r#type;
mod version;

pub use auto::Auto;
pub use datetime::{DateTime, DateTimeBuilder};
pub use duration::{Duration, DurationBuilder};
pub use path::Path;
pub use symbol::Symbol;
pub use r#type::Type;
pub use version::Version;
