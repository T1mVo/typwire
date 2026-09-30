//! Wire formats: one submodule per format, each implementing [`crate::Format`].
//! `Cbor` is unconditional and the default everywhere -- see `wire.rs`'s
//! `encode`/`decode`/Typst's own `typwire.call`; `Json` is behind its own
//! crate feature -- see `Cargo.toml`.

mod cbor;
#[cfg(feature = "json")]
mod json;
// Unconditional, unlike `json` below: `from_value` backs
// `wire.rs::value_to_owned`, which `any.rs` calls to reconstruct a native
// type nested inside an `Any` -- needed even in a CBOR-only build, not just
// when the `json` feature is on.
mod value;

pub use cbor::Cbor;
#[cfg(feature = "json")]
pub use json::Json;
pub(crate) use value::from_value;
#[cfg(feature = "json")]
pub(crate) use value::to_value;
