#import "../../../utils.typ": is-debug-assert

#let type-name = "color-oklab"

/// Encode an OKLab color into a wire-compatible dictionary.
///
/// - lightness (ratio): The lightness component of the color.
/// - a (float): The 'a' component of the color.
/// - b (float): The 'b' component of the color.
/// - alpha (ratio): The alpha (transparency) component of the color.
/// - format (str): The wire format being encoded for. Unused -- `oklab`
///   has the same tagged-dictionary shape in every format.
/// -> dictionary
#let encode(lightness, a, b, alpha, format: "cbor") = {
  if is-debug-assert {
    assert(
      type(lightness) == std.ratio,
      message: "oklab.encode: lightness must be of type ratio",
    )
    assert(
      type(a) == std.float,
      message: "oklab.encode: a must be of type float",
    )
    assert(
      type(b) == std.float,
      message: "oklab.encode: b must be of type float",
    )
    assert(
      type(alpha) == std.ratio,
      message: "oklab.encode: alpha must be of type ratio",
    )
  }

  (
    typwire-type: type-name,
    lightness: lightness,
    a: a,
    b: b,
    alpha: alpha,
  )
}

/// Decode a wire-compatible dictionary into an OKLab color.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// - format (str): The wire format being decoded from. Unused -- `oklab`
///   decodes the same way regardless of format.
/// -> color
#let decode(value, format: "cbor") = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "oklab.decode: value must be a tagged with type "
        + type-name
        + ". Found "
        + value.typwire-type,
    )
    assert(
      type(value.lightness) == std.ratio,
      message: "oklab.decode: value.lightness must be of type ratio, Found "
        + str(type(value.lightness)),
    )
    assert(
      type(value.a) == std.float,
      message: "oklab.decode: value.a must be of type float, Found "
        + str(type(value.a)),
    )
    assert(
      type(value.b) == std.float,
      message: "oklab.decode: value.b must be of type float, Found "
        + str(type(value.b)),
    )
    assert(
      type(value.alpha) == std.ratio,
      message: "oklab.decode: value.alpha must be of type ratio, Found "
        + str(type(value.alpha)),
    )
  }

  color.oklab(value.lightness, value.a, value.b, value.alpha)
}
