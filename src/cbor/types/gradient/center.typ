#import "../../utils.typ": is-debug-assert

/// Encode a center point into a CBOR-compatible dictionary.
///
/// - x (ratio): A ratio value between 0% and 100%.
/// - y (ratio): A ratio value between 0% and 100%.
/// -> dictionary
#let encode(x, y) = {
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

/// Decode a CBOR-compatible dictionary into a center point tuple.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// -> (float, float)
#let decode(value) = {
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
