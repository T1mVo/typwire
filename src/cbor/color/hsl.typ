#import "../utils.typ": is-debug-assert

#let type-name = "color-hsl"

/// Encode an HSL color into a CBOR-compatible dictionary.
///
/// - hue (angle): The hue component of the color.
/// - saturation (ratio): The saturation component of the color.
/// - lightness (ratio): The lightness component of the color.
/// - alpha (ratio): The alpha (transparency) component of the color.
/// -> dictionary
#let encode(hue, saturation, lightness, alpha) = {
  if is-debug-assert {
    assert(
      type(hue) == std.angle,
      message: "hsl.encode: hue must be of type angle",
    )
    assert(
      type(saturation) == std.ratio,
      message: "hsl.encode: saturation must be of type ratio",
    )
    assert(
      type(lightness) == std.ratio,
      message: "hsl.encode: lightness must be of type ratio",
    )
    assert(
      type(alpha) == std.ratio,
      message: "hsl.encode: alpha must be of type ratio",
    )
  }

  (
    typwire-type: type-name,
    hue: hue,
    saturation: saturation,
    lightness: lightness,
    alpha: alpha,
  )
}

/// Decode a CBOR-compatible dictionary into an HSL color.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// -> color
#let decode(value) = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "hsl.decode: value must be a tagged with type "
        + type-name
        + ". Found "
        + value.typwire-type,
    )
    assert(
      type(value.hue) == std.angle,
      message: "hsl.decode: value.hue must be of type angle, Found "
        + str(type(value.hue)),
    )
    assert(
      type(value.saturation) == std.ratio,
      message: "hsl.decode: value.saturation must be of type ratio, Found "
        + str(type(value.saturation)),
    )
    assert(
      type(value.lightness) == std.ratio,
      message: "hsl.decode: value.lightness must be of type ratio, Found "
        + str(type(value.lightness)),
    )
    assert(
      type(value.alpha) == std.ratio,
      message: "hsl.decode: value.alpha must be of type ratio, Found "
        + str(type(value.alpha)),
    )
  }

  color.hsl(value.hue, value.saturation, value.lightness, value.alpha)
}
