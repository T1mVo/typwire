#import "../utils.typ": is-debug-assert

#let type-name = "color-oklch"

/// Encode an OKLCH color into a CBOR-compatible dictionary.
///
/// - lightness (ratio): The lightness component of the color.
/// - chroma (float): The chroma component of the color.
/// - hue (angle): The hue component of the color.
/// - alpha (ratio): The alpha (transparency) component of the color.
/// -> dictionary
#let encode(lightness, chroma, hue, alpha) = {
  if is-debug-assert {
    assert(
      type(lightness) == std.ratio,
      message: "oklch.encode: lightness must be of type ratio",
    )
    assert(
      type(chroma) == std.float,
      message: "oklch.encode: chroma must be of type float",
    )
    assert(
      type(hue) == std.angle,
      message: "oklch.encode: hue must be of type angle",
    )
    assert(
      type(alpha) == std.ratio,
      message: "oklch.encode: alpha must be of type ratio",
    )
  }

  (
    typwire-type: type-name,
    lightness: lightness,
    chroma: chroma,
    hue: hue,
    alpha: alpha,
  )
}

/// Decode a CBOR-compatible dictionary into an OKLCH color.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// -> color
#let decode(value) = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "oklch.decode: value must be a tagged with type "
        + type-name
        + ". Found "
        + value.typwire-type,
    )
    assert(
      type(value.lightness) == std.ratio,
      message: "oklch.decode: value.lightness must be of type ratio, Found "
        + str(type(value.lightness)),
    )
    assert(
      type(value.chroma) == std.float,
      message: "oklch.decode: value.chroma must be of type float, Found "
        + str(type(value.chroma)),
    )
    assert(
      type(value.hue) == std.angle,
      message: "oklch.decode: value.hue must be of type angle, Found "
        + str(type(value.hue)),
    )
    assert(
      type(value.alpha) == std.ratio,
      message: "oklch.decode: value.alpha must be of type ratio, Found "
        + str(type(value.alpha)),
    )
  }

  color.oklch(value.lightness, value.chroma, value.hue, value.alpha)
}
