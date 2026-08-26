#import "../utils.typ": is-debug-assert

#let type-name = "color-cmyk"

/// Encode a CMYK color into a CBOR-compatible dictionary.
///
/// - cyan (ratio): The cyan component of the color.
/// - magenta (ratio): The magenta component of the color.
/// - yellow (ratio): The yellow component of the color.
/// - key (ratio): The key (black) component of the color.
/// -> dictionary
#let encode(cyan, magenta, yellow, key) = {
  if is-debug-assert {
    assert(
      type(cyan) == std.ratio,
      message: "cmyk.encode: cyan must be of type ratio",
    )
    assert(
      type(magenta) == std.ratio,
      message: "cmyk.encode: magenta must be of type ratio",
    )
    assert(
      type(yellow) == std.ratio,
      message: "cmyk.encode: yellow must be of type ratio",
    )
    assert(
      type(key) == std.ratio,
      message: "cmyk.encode: key must be of type ratio",
    )
  }

  (
    typwire-type: type-name,
    cyan: cyan,
    magenta: magenta,
    yellow: yellow,
    key: key,
  )
}

/// Decode a CBOR-compatible dictionary into a CMYK color.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// -> color
#let decode(value) = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "cmyk.decode: value must be a tagged with type "
        + type-name
        + ". Found "
        + value.typwire-type,
    )
    assert(
      type(value.cyan) == std.ratio,
      message: "cmyk.decode: value.cyan must be of type ratio, Found "
        + str(type(value.cyan)),
    )
    assert(
      type(value.magenta) == std.ratio,
      message: "cmyk.decode: value.magenta must be of type ratio, Found "
        + str(type(value.magenta)),
    )
    assert(
      type(value.yellow) == std.ratio,
      message: "cmyk.decode: value.yellow must be of type ratio, Found "
        + str(type(value.yellow)),
    )
    assert(
      type(value.key) == std.ratio,
      message: "cmyk.decode: value.key must be of type ratio, Found "
        + str(type(value.key)),
    )
  }

  color.cmyk(value.cyan, value.magenta, value.yellow, value.key)
}