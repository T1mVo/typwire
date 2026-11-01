//! Wire formats: one submodule per format, each implementing [`crate::Format`].
//! `Cbor` is unconditional and the default everywhere -- see `wire.rs`'s
//! `encode`/`decode`/Typst's own `typwire.call`; `Json` is behind its own
//! crate feature -- see `Cargo.toml`.

mod cbor;
#[cfg(feature = "json")]
mod json;
// `Value` tree (de)serialization -- only `Json` needs it (see its own doc
// comment); `Cbor`, `Any` included, decodes straight from the bytes.
#[cfg(feature = "json")]
mod value;

pub use cbor::Cbor;
#[cfg(feature = "json")]
pub use json::Json;
#[cfg(feature = "json")]
pub(crate) use value::{from_value, to_value};
