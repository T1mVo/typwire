#import "../wire/lib.typ" as wire

/// Encode a value into a YAML string.
///
/// YAML has no native byte-string type, so any `bytes` value is tagged
/// into an array instead -- handled directly by `wire.encode(value,
/// "yaml")`, in the same recursive pass as everything else.
///
/// - value (any): The value to encode.
/// -> str
#let encode(value) = yaml.encode(wire.encode(value, format: "yaml"))

/// Decode a YAML string or bytes and reconstruct supported native Typst
/// values.
///
/// A plain `str` is coerced to `bytes` first -- Typst's own `yaml(...)`
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
  wire.decode(yaml(data), format: "yaml")
}

/// Call a typed Typwire plugin function with positional arguments, using
/// YAML as the wire format.
///
/// Each argument is encoded independently and the returned bytes are
/// decoded. Plugin calls require `bytes` arguments, so each encoded string
/// is wrapped with `bytes(..)` before the call.
#let call(function, ..args) = {
  if args.named().len() != 0 {
    panic("typwire.yaml.call: named arguments are not supported")
  }

  let encoded = args.pos().map(value => bytes(encode(value)))
  decode(function(..encoded))
}
