#import "../wire/lib.typ" as wire

/// Encode a value into a TOML string.
///
/// TOML has no native byte-string type and no `none`/null, so `bytes` and
/// `none` are both tagged into other shapes instead -- handled directly by
/// `wire.encode(value, "toml")`, in the same recursive pass as everything
/// else. TOML's own grammar also requires a top-level table, unlike
/// CBOR/JSON/YAML, which can encode any shape at the root -- so,
/// unconditionally, the whole tree is wrapped under one fixed key
/// (`"value"`) before reaching `toml.encode`, the same way on both sides of
/// the wire (see the mirrored envelope in `typwire::Toml::encode`,
/// `src/core/crates/typwire/src/format/toml.rs`). This makes every value
/// shape, scalars and `none` included, representable in TOML -- there is no
/// value `typwire.toml` can't round-trip.
///
/// - value (any): The value to encode.
/// -> str
#let encode(value) = toml.encode((value: wire.encode(value, format: "toml")))

/// Decode a TOML string or bytes and reconstruct supported native Typst
/// values.
///
/// A plain `str` is coerced to `bytes` first -- Typst's own `toml(...)`
/// treats a bare `str` argument as a *file path* to load, not inline
/// content, so passing `encode`'s own `str` output straight through would
/// silently try to read a file instead of decoding it.
///
/// - data (bytes, str): The encoded Typwire value.
/// -> any
#let decode(data) = {
  if std.type(data) == str {
    data = bytes(data)
  }
  wire.decode(toml(data).value, format: "toml")
}

/// Call a typed Typwire plugin function with positional arguments, using
/// TOML as the wire format.
///
/// Each argument is encoded independently and the returned bytes are
/// decoded. Plugin calls require `bytes` arguments, so each encoded string
/// is wrapped with `bytes(..)` before the call.
#let call(function, ..args) = {
  if args.named().len() != 0 {
    panic("typwire.toml.call: named arguments are not supported")
  }

  let encoded = args.pos().map(value => bytes(encode(value)))
  decode(function(..encoded))
}
