#import "../../../utils.typ": is-debug-assert

/// Encode a stop into a wire-compatible dictionary.
/// -> dictionary
#let encode(
  /// A color.
  /// -> color
  color,
  /// A ratio value between 0% and 100%.
  /// -> ratio
  offset,
  /// The wire format being encoded for. Unused -- `stop` has the same
  /// untagged-dictionary shape in every format.
  /// -> str
  format: "cbor",
) = {
  if is-debug-assert {
    assert(
      type(color) == std.color,
      message: "stop.encode: color must be of type color",
    )
    assert(
      type(offset) == std.ratio,
      message: "stop.encode: offset must be of type ratio",
    )
  }

  (
    color: color,
    offset: offset,
  )
}

/// Decode a wire-compatible dictionary into a stop tuple.
/// -> (color, float)
#let decode(
  /// A dictionary produced by `encode`.
  /// -> dictionary
  value,
  /// The wire format being decoded from. Unused -- `stop` decodes the same
  /// way regardless of format.
  /// -> str
  format: "cbor",
) = {
  if is-debug-assert {
    assert(
      type(value.color) == std.color,
      message: "stop.decode: value.color must be of type color, Found "
        + str(type(value.color)),
    )
    assert(
      type(value.offset) == std.ratio,
      message: "stop.decode: value.offset must be of type ratio, Found "
        + str(type(value.offset)),
    )
  }

  (value.color, value.offset)
}
