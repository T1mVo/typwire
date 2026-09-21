#import "../../../utils.typ": is-debug-assert

/// Encode a stop into a CBOR-compatible dictionary.
///
/// - color (color): A color,
/// - offset (ratio): A ratio value between 0% and 100%.
/// -> dictionary
#let encode(color, offset) = {
  if is-debug-assert {
    assert(
      type(color) == std.color,
      message: "stop.encode: color must be of type color",
    )
    assert(
      type(offset) == std.ratio,
      message: "stop.encode: offset must be of type ratio",
    )
  }

  (
    color: color,
    offset: offset,
  )
}

/// Decode a CBOR-compatible dictionary into a stop tuple.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// -> (color, float)
#let decode(value) = {
  if is-debug-assert {
    assert(
      type(value.color) == std.color,
      message: "stop.decode: value.color must be of type color, Found "
        + str(type(value.color)),
    )
    assert(
      type(value.offset) == std.ratio,
      message: "stop.decode: value.offset must be of type ratio, Found "
        + str(type(value.offset)),
    )
  }

  (value.color, value.offset)
}
