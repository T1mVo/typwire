#import "../../utils.typ": is-debug-assert

#let type-name = "label"

/// Encode a label into a CBOR-compatible dictionary.
///
/// - value (label): The label to encode.
/// -> dictionary
#let encode(value) = {
  if is-debug-assert {
    assert(
      type(value) == label,
      message: "label.encode: value must be of type label",
    )
  }

  (
    typwire-type: type-name,
    name: str(value),
  )
}

/// Decode a CBOR-compatible dictionary into a label.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// -> label
#let decode(value) = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "label.decode: value must be a tagged with type " + type-name + ". Found " + value.typwire-type,
    )
    assert(
      type(value.name) == str,
      message: "label.decode: value.name must be of type string, Found " + str(type(value.name)),
    )
  }

  label(value.name)
}
