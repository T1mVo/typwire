#import "../../../utils.typ": is-debug-assert

#let type-name = "color-luma"

/// Encode a luma color into a wire-compatible dictionary.
///
/// - lightness (ratio): The lightness component of the color.
/// - alpha (ratio): The alpha (transparency) component of the color.
/// - format (str): The wire format being encoded for. Unused -- `luma` has
///   the same tagged-dictionary shape in every format.
/// -> dictionary
#let encode(lightness, alpha, format: "cbor") = {
  if is-debug-assert {
    assert(
      type(lightness) == std.ratio,
      message: "luma.encode: lightness must be of type ratio",
    )
    assert(
      type(alpha) == std.ratio,
      message: "luma.encode: alpha must be of type ratio",
    )
  }

  (
    typwire-type: type-name,
    lightness: lightness,
    alpha: alpha,
  )
}

/// Decode a wire-compatible dictionary into a luma color.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// - format (str): The wire format being decoded from. Unused -- `luma`
///   decodes the same way regardless of format.
/// -> color
#let decode(value, format: "cbor") = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "luma.decode: value must be a tagged with type "
        + type-name
        + ". Found "
        + value.typwire-type,
    )
    assert(
      type(value.lightness) == std.ratio,
      message: "luma.decode: value.lightness must be of type ratio, Found "
        + str(type(value.lightness)),
    )
    assert(
      type(value.alpha) == std.ratio,
      message: "luma.decode: value.alpha must be of type ratio, Found "
        + str(type(value.alpha)),
    )
  }

  color.luma(value.lightness, value.alpha)
}
