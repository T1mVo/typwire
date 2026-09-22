#import "../../../utils.typ": is-debug-assert

#let type-name = "color-hsv"

/// Encode an HSV color into a wire-compatible dictionary.
///
/// - hue (angle): The hue component of the color.
/// - saturation (ratio): The saturation component of the color.
/// - value (ratio): The value (brightness) component of the color.
/// - alpha (ratio): The alpha (transparency) component of the color.
/// - format (str): The wire format being encoded for. Unused -- `hsv` has
///   the same tagged-dictionary shape in every format.
/// -> dictionary
#let encode(hue, saturation, value, alpha, format: "cbor") = {
  if is-debug-assert {
    assert(
      type(hue) == std.angle,
      message: "hsv.encode: hue must be of type angle",
    )
    assert(
      type(saturation) == std.ratio,
      message: "hsv.encode: saturation must be of type ratio",
    )
    assert(
      type(value) == std.ratio,
      message: "hsv.encode: value must be of type ratio",
    )
    assert(
      type(alpha) == std.ratio,
      message: "hsv.encode: alpha must be of type ratio",
    )
  }

  (
    typwire-type: type-name,
    hue: hue,
    saturation: saturation,
    value: value,
    alpha: alpha,
  )
}

/// Decode a wire-compatible dictionary into an HSV color.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// - format (str): The wire format being decoded from. Unused -- `hsv`
///   decodes the same way regardless of format.
/// -> color
#let decode(value, format: "cbor") = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "hsv.decode: value must be a tagged with type "
        + type-name
        + ". Found "
        + value.typwire-type,
    )
    assert(
      type(value.hue) == std.angle,
      message: "hsv.decode: value.hue must be of type angle, Found "
        + str(type(value.hue)),
    )
    assert(
      type(value.saturation) == std.ratio,
      message: "hsv.decode: value.saturation must be of type ratio, Found "
        + str(type(value.saturation)),
    )
    assert(
      type(value.value) == std.ratio,
      message: "hsv.decode: value.value must be of type ratio, Found "
        + str(type(value.value)),
    )
    assert(
      type(value.alpha) == std.ratio,
      message: "hsv.decode: value.alpha must be of type ratio, Found "
        + str(type(value.alpha)),
    )
  }

  color.hsv(value.hue, value.saturation, value.value, value.alpha)
}
