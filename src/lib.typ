#import "wire/lib.typ" as wire
#import "format/cbor.typ" as cbor
#import "format/json.typ" as json
#import "format/toml.typ" as toml
#import "format/yaml.typ" as yaml

/// Encode a Typst value into the shared, format-agnostic dictionary/array/
/// primitive tree every `typwire.<format>.encode` wraps with a native
/// format encoder. Exposed for advanced/manual use; most callers want
/// `typwire.call` or one of the format namespaces below instead.
#let encode = wire.encode

/// The mirror of `encode`: reconstruct native Typst values from a tree
/// already decoded by a native format decoder.
#let decode = wire.decode

/// Call a typed Typwire plugin function with positional arguments, using
/// CBOR as the wire format -- the same default `#[typwire::export]` picks
/// on the Rust side. Use `typwire.json.call`/`typwire.yaml.call`/
/// `typwire.toml.call` instead for a plugin function exported with a
/// different `format`.
///
/// Each argument is encoded independently and the returned bytes are decoded.
#let call = cbor.call
