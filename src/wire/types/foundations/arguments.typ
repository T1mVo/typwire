#import "../../utils.typ": is-debug-assert

#let type-name = "arguments"

/// Encode captured arguments into a CBOR-compatible dictionary.
///
/// - value (arguments): Captured arguments.
/// -> dictionary
#let encode(value) = {
  if is-debug-assert {
    assert(
      type(value) == arguments,
      message: "arguments.encode: value must be of type arguments",
    )
  }

  (
    typwire-type: type-name,
    pos: value.pos(),
    named: value.named(),
  )
}

/// Decode a CBOR-compatible dictionary into captured arguments.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// -> arguments
#let decode(value) = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "arguments.decode: value must be a tagged with type " + type-name + ". Found " + value.typwire-type,
    )
    assert(
      type(value.pos) == array,
      message: "arguments.decode: value.pos must be of type array, Found " + str(type(value.pos)),
    )
    assert(
      type(value.named) == dictionary,
      message: "arguments.decode: value.named must be of type dictionary, Found " + str(type(value.named)),
    )
  }

  arguments(..value.pos, ..value.named)
}
