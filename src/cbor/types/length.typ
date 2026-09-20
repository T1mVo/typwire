#import "../utils.typ": float-to-length, is-debug-assert

#let type-name = "length"

/// Encode a length into a CBOR-compatible dictionary.
///
/// - length (length): A length.
/// -> dictionary
#let encode(length) = {
  if is-debug-assert {
    assert(
      type(length) == std.length,
      message: "length.encode: length must be of type length",
    )
  }

  (
    typwire-type: type-name,
    points: length.pt(),
  )
}

/// Decode a CBOR-compatible dictionary into a length.
///
/// - value (dictionary): A dictionary representing the `encode`d length.
/// -> length
#let decode(value) = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "length.decode: value must be a tagged with type "
        + type-name
        + ". Found "
        + value.typwire-type,
    )
    assert(
      type(value.points) == std.float,
      message: "length.decode: value.points must be of type float",
    )
  }

  float-to-length(value.points)
}
