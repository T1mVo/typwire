#import "format/cbor.typ" as cbor
#import "format/debug.typ" as debug
#import "format/json.typ" as json
#import "format/toml.typ" as toml
#import "format/yaml.typ" as yaml

/// Encode a Typst value into CBOR bytes -- the same default wire format
/// `#[typwire::export]` picks on the Rust side. Use
/// `typwire.json.encode`/`typwire.yaml.encode`/`typwire.toml.encode`
/// instead for a plugin function exported with a different `format`, or
/// `typwire.debug.encode` to inspect the format-agnostic tree with no
/// serialization at all.
#let encode = cbor.encode

/// The mirror of `encode`: decode CBOR bytes and reconstruct supported
/// native Typst values.
#let decode = cbor.decode

/// Call a typed Typwire plugin function with positional arguments, using
/// CBOR as the wire format -- the same default `#[typwire::export]` picks
/// on the Rust side. Use `typwire.json.call`/`typwire.yaml.call`/
/// `typwire.toml.call` instead for a plugin function exported with a
/// different `format`.
///
/// Each argument is encoded independently and the returned bytes are decoded.
#let call = cbor.call
