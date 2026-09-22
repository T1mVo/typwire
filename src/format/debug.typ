#import "../wire/lib.typ" as wire

/// Encode a value into the shared dictionary/array/primitive tree
/// `typwire.<format>.encode` would otherwise hand to a native format
/// encoder -- with no serialization step applied, so the result is a plain
/// Typst value, not `bytes`/`str`. Useful for inspecting or testing the
/// wire shape Typwire produces without a plugin round trip.
///
/// `bytes`/`none` are the one part of this tree that's still
/// format-sensitive (only CBOR keeps them native; every other format tags
/// them), so `format` picks which format's tagging rules to preview --
/// it's otherwise unrelated to serialization, which this namespace never
/// does.
///
/// - value (any): The value to encode.
/// - format (str): Which format's tagging rules to preview for `bytes`/
///   `none`. One of `"cbor"`, `"json"`, `"yaml"`, `"toml"`.
/// -> any
#let encode(value, format: "cbor") = wire.encode(value, format: format)

/// The mirror of `encode`: reconstruct native Typst values from a tree that
/// was never serialized, such as one produced by `encode` above.
///
/// - data (any): A dictionary/array/primitive tree, as produced by `encode`.
/// - format (str): The format `data`'s tree was produced for -- must match
///   whatever `encode` (or a real `typwire.<format>.encode`) used to build
///   it, so `bytes`/`none` tags are resolved the same way they were tagged.
/// -> any
#let decode(data, format: "cbor") = wire.decode(data, format: format)
