#import "../../../utils.typ": is-debug-assert

#let type-name = "color-cmyk"

/// Encode a CMYK color into a wire-compatible dictionary.
/// -> dictionary
#let encode(
  /// The cyan component of the color.
  /// -> ratio
  cyan,
  /// The magenta component of the color.
  /// -> ratio
  magenta,
  /// The yellow component of the color.
  /// -> ratio
  yellow,
  /// The key (black) component of the color.
  /// -> ratio
  key,
  /// The wire format being encoded for. Unused -- `cmyk` has the same
  /// tagged-dictionary shape in every format.
  /// -> str
  format: "cbor",
) = {
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

/// Decode a wire-compatible dictionary into a CMYK color.
/// -> color
#let decode(
  /// A dictionary produced by `encode`.
  /// -> dictionary
  value,
  /// The wire format being decoded from. Unused -- `cmyk` decodes the same
  /// way regardless of format.
  /// -> str
  format: "cbor",
) = {
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
