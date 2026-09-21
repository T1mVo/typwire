//! Wire formats other than the default (`Cbor`, in `wire.rs`), each behind
//! its own crate feature -- see `Cargo.toml`.

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "toml")]
mod toml;
#[cfg(feature = "yaml")]
mod yaml;

#[cfg(feature = "json")]
pub use json::Json;
#[cfg(feature = "toml")]
pub use toml::Toml;
#[cfg(feature = "yaml")]
pub use yaml::Yaml;
