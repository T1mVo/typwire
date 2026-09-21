//! Wire formats: one submodule per format, each implementing [`crate::Format`].
//! `Cbor` is unconditional and the default everywhere -- see `wire.rs`'s
//! `encode`/`decode`/Typst's own `typwire.call`; `Json`/`Yaml`/`Toml` are
//! each behind their own crate feature -- see `Cargo.toml`.

mod cbor;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "toml")]
mod toml;
#[cfg(feature = "yaml")]
mod yaml;

pub use cbor::Cbor;
#[cfg(feature = "json")]
pub use json::Json;
#[cfg(feature = "toml")]
pub use toml::Toml;
#[cfg(feature = "yaml")]
pub use yaml::Yaml;
