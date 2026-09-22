#import "../../../utils.typ": is-debug-assert

/// Encode a center point into a wire-compatible dictionary.
///
/// - x (ratio): A ratio value between 0% and 100%.
/// - y (ratio): A ratio value between 0% and 100%.
/// - format (str): The wire format being encoded for. Unused -- `center`
///   has the same untagged-dictionary shape in every format.
/// -> dictionary
#let encode(x, y, format: "cbor") = {
  if is-debug-assert {
    assert(
      type(x) == std.ratio,
      message: "center.encode: x must be of type ratio",
    )
    assert(
      type(y) == std.ratio,
      message: "center.encode: y must be of type ratio",
    )
  }

  (
    x: x,
    y: y,
  )
}

/// Decode a wire-compatible dictionary into a center point tuple.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// - format (str): The wire format being decoded from. Unused -- `center`
///   decodes the same way regardless of format.
/// -> (float, float)
#let decode(value, format: "cbor") = {
  if is-debug-assert {
    assert(
      type(value.x) == std.ratio,
      message: "center.decode: value.x must be of type ratio, Found "
        + str(type(value.x)),
    )
    assert(
      type(value.y) == std.ratio,
      message: "center.decode: value.y must be of type ratio, Found "
        + str(type(value.y)),
    )
  }

  (value.x, value.y)
}
