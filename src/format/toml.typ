#import "../wire/lib.typ" as wire
#import "../wire/rewrite-bytes.typ": bytes-to-array

/// Encode a value into a TOML string.
///
/// TOML has no native byte-string type, so any `bytes` value is rewritten
/// into a tagged array first -- see `wire/rewrite-bytes.typ`. TOML's own
/// grammar requires a top-level table: encoding a value that isn't
/// dictionary-shaped (a bare `int`, an `array`, ...) fails with a clear
/// error from `toml.encode` itself.
///
/// - value (any): The value to encode.
/// -> str
#let encode(value) = toml.encode(bytes-to-array(wire.encode(value)))

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
  wire.decode(toml(data))
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
