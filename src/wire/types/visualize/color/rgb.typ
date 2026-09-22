#import "../../../utils.typ": is-debug-assert

#let type-name = "color-rgb"

/// Encode an RGB color into a wire-compatible dictionary.
///
/// - r (ratio): The red component of the color.
/// - g (ratio): The green component of the color.
/// - b (ratio): The blue component of the color.
/// - alpha (ratio): The alpha (transparency) component of the color.
/// - format (str): The wire format being encoded for. Unused -- `rgb` has
///   the same tagged-dictionary shape in every format.
/// -> dictionary
#let encode(r, g, b, alpha, format: "cbor") = {
  if is-debug-assert {
    assert(
      type(r) == std.ratio,
      message: "rgb.encode: r must be of type ratio",
    )
    assert(
      type(g) == std.ratio,
      message: "rgb.encode: g must be of type ratio",
    )
    assert(
      type(b) == std.ratio,
      message: "rgb.encode: b must be of type ratio",
    )
    assert(
      type(alpha) == std.ratio,
      message: "rgb.encode: alpha must be of type ratio",
    )
  }

  (
    typwire-type: type-name,
    r: r,
    g: g,
    b: b,
    alpha: alpha,
  )
}

/// Decode a wire-compatible dictionary into an RGB color.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// - format (str): The wire format being decoded from. Unused -- `rgb`
///   decodes the same way regardless of format.
/// -> color
#let decode(value, format: "cbor") = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "rgb.decode: value must be a tagged with type "
        + type-name
        + ". Found "
        + value.typwire-type,
    )
    assert(
      type(value.r) == std.ratio,
      message: "rgb.decode: value.r must be of type ratio, Found "
        + str(type(value.r)),
    )
    assert(
      type(value.g) == std.ratio,
      message: "rgb.decode: value.g must be of type ratio, Found "
        + str(type(value.g)),
    )
    assert(
      type(value.b) == std.ratio,
      message: "rgb.decode: value.b must be of type ratio, Found "
        + str(type(value.b)),
    )
    assert(
      type(value.alpha) == std.ratio,
      message: "rgb.decode: value.alpha must be of type ratio, Found "
        + str(type(value.alpha)),
    )
  }

  color.rgb(value.r, value.g, value.b, value.alpha)
}
