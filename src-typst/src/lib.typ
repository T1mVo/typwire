#import "cbor/lib.typ" as cbor
#import "cbor/lib.typ": encode, decode

/// Call a typed Typwire plugin function with positional arguments.
///
/// Each argument is encoded independently and the returned bytes are decoded.
#let call(function, ..args) = {
  if args.named().len() != 0 {
    panic("typwire.call: named arguments are not supported")
  }

  let encoded = args.pos().map(encode)
  decode(function(..encoded))
}
