//! Types from Typst's `foundations` category: <https://typst.app/docs/reference/foundations/>.

mod auto;
mod datetime;
mod duration;
mod r#type;
mod version;

pub use auto::Auto;
pub use datetime::{DateTime, DateTimeBuilder};
pub use duration::{Duration, DurationBuilder};
pub use r#type::Type;
pub use version::Version;
