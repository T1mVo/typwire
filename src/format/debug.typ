#import "../wire/lib.typ" as wire

/// Encode a value into the shared, format-agnostic dictionary/array/
/// primitive tree `typwire.<format>.encode` would otherwise hand to a
/// native format encoder -- with no serialization step applied, so the
/// result is a plain Typst value, not `bytes`/`str`. Useful for inspecting
/// or testing the wire shape Typwire produces without a plugin round trip.
///
/// - value (any): The value to encode.
/// -> any
#let encode = wire.encode

/// The mirror of `encode`: reconstruct native Typst values from a tree that
/// was never serialized, such as one produced by `encode` above.
///
/// - data (any): A dictionary/array/primitive tree, as produced by `encode`.
/// -> any
#let decode = wire.decode
