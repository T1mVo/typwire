#import "../utils.typ": is-debug-assert

#let type-name = "color-linear-rgb"

/// Encode a linear RGB color into a CBOR-compatible dictionary.
///
/// - r (ratio): The red component of the color.
/// - g (ratio): The green component of the color.
/// - b (ratio): The blue component of the color.
/// - alpha (ratio): The alpha (transparency) component of the color.
/// -> dictionary
#let encode(r, g, b, alpha) = {
  if is-debug-assert {
    assert(
      type(r) == std.ratio,
      message: "linear-rgb.encode: r must be of type ratio",
    )
    assert(
      type(g) == std.ratio,
      message: "linear-rgb.encode: g must be of type ratio",
    )
    assert(
      type(b) == std.ratio,
      message: "linear-rgb.encode: b must be of type ratio",
    )
    assert(
      type(alpha) == std.ratio,
      message: "linear-rgb.encode: alpha must be of type ratio",
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

/// Decode a CBOR-compatible dictionary into a linear RGB color.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// -> color
#let decode(value) = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "linear-rgb.decode: value must be a tagged with type "
        + type-name
        + ". Found "
        + value.typwire-type,
    )
    assert(
      type(value.r) == std.ratio,
      message: "linear-rgb.decode: value.r must be of type ratio, Found "
        + str(type(value.r)),
    )
    assert(
      type(value.g) == std.ratio,
      message: "linear-rgb.decode: value.g must be of type ratio, Found "
        + str(type(value.g)),
    )
    assert(
      type(value.b) == std.ratio,
      message: "linear-rgb.decode: value.b must be of type ratio, Found "
        + str(type(value.b)),
    )
    assert(
      type(value.alpha) == std.ratio,
      message: "linear-rgb.decode: value.alpha must be of type ratio, Found "
        + str(type(value.alpha)),
    )
  }

  color.linear-rgb(value.r, value.g, value.b, value.alpha)
}
