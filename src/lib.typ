#import "format/cbor.typ" as cbor-format
#import "format/debug.typ" as debug
#import "format/json.typ" as json-format

// Maps a format name to the module that implements it. `encode`/`decode`/
// `call` below are the single place that actually dispatches on `format`;
// every `typwire.<format>` namespace further down is just a fixed-format
// wrapper around them.
#let formats = (
  cbor: cbor-format,
  json: json-format,
)

#let resolve-format(format) = {
  let module = formats.at(format, default: none)
  if module == none {
    panic(
      "typwire: unknown format \"" + format + "\" "
        + "(expected one of \"cbor\", \"json\")",
    )
  }
  module
}

/// Encode a Typst value into a wire format's bytes/string.
///
/// - value (any): The value to encode.
/// - format (str): The wire format to encode into -- one of `"cbor"` (the
///   default, matching `#[typwire::export]`'s own default), `"json"`. Use
///   `typwire.debug.encode` instead to inspect the shared tree with no
///   serialization at all.
/// -> bytes, str
#let encode(value, format: "cbor") = resolve-format(format).encode(value)

/// The mirror of `encode`: decode a wire format's bytes/string and
/// reconstruct supported native Typst values.
///
/// - data (bytes, str): The encoded Typwire value.
/// - format (str): The wire format `data` was encoded with -- must match.
/// -> any
#let decode(data, format: "cbor") = resolve-format(format).decode(data)

/// Call a typed Typwire plugin function with positional arguments, using
/// the given wire format -- the plugin export and the call site must agree
/// on the same one.
///
/// Each argument is encoded independently and the returned bytes are
/// decoded.
///
/// - function (function): The plugin function to call.
/// - ..args (arguments): Positional arguments to encode and pass.
/// - format (str): The wire format to call with -- one of `"cbor"` (the
///   default, matching `#[typwire::export]`'s own default), `"json"`.
/// -> any
#let call(function, ..args, format: "cbor") = resolve-format(format).call(function, ..args)

// Each namespace below re-exports the real per-format module -- the exact
// module `formats` above points `encode`/`decode`/`call`'s dispatch at, so
// `typwire.<format>.encode(value)` and `typwire.encode(value, format:
// "<format>")` run the identical function either way. Re-exported directly
// (not wrapped in a plain dictionary of closures) so `typwire.<format>.call(
// ..)`'s dot-call syntax keeps working -- Typst allows that sugar for a
// module's own fields, but not for a dictionary value that merely holds a
// function.
#let cbor = cbor-format
#let json = json-format
