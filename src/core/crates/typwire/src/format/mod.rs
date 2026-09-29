//! Wire formats: one submodule per format, each implementing [`crate::Format`].
//! `Cbor` is unconditional and the default everywhere -- see `wire.rs`'s
//! `encode`/`decode`/Typst's own `typwire.call`; `Json`/`Yaml`/`Toml` are
//! each behind their own crate feature -- see `Cargo.toml`.

mod cbor;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "toml")]
mod toml;
// Unconditional, unlike `json`/`toml`/`yaml` below: `from_value` backs
// `wire.rs::value_to_owned`, which `any.rs` calls to reconstruct a native
// type nested inside an `Any` -- needed even in a CBOR-only build, not just
// when a text format's feature is on.
mod value;
#[cfg(feature = "yaml")]
mod yaml;

pub use cbor::Cbor;
#[cfg(feature = "json")]
pub use json::Json;
#[cfg(feature = "toml")]
pub use toml::Toml;
pub(crate) use value::from_value;
#[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
pub(crate) use value::to_value;
#[cfg(feature = "yaml")]
pub use yaml::Yaml;
