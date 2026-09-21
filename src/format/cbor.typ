#import "../wire/lib.typ" as wire

/// Encode a value into CBOR bytes.
///
/// - value (any): The value to encode.
/// -> bytes
#let encode(value) = cbor.encode(wire.encode(value))

/// Decode CBOR bytes and reconstruct supported native Typst values.
///
/// - data (bytes): The encoded Typwire value.
/// -> any
#let decode(data) = {
  if std.type(data) != bytes {
    panic("typwire.cbor.decode: data must be bytes")
  }
  wire.decode(cbor(data))
}

/// Call a typed Typwire plugin function with positional arguments, using
/// CBOR as the wire format.
///
/// Each argument is encoded independently and the returned bytes are decoded.
#let call(function, ..args) = {
  if args.named().len() != 0 {
    panic("typwire.cbor.call: named arguments are not supported")
  }

  let encoded = args.pos().map(encode)
  decode(function(..encoded))
}
