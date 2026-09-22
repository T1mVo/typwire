#import "../../utils.typ": float-to-ratio, is-debug-assert, ratio-to-float

#let type-name = "ratio"

/// Encode a ratio into a wire-compatible float.
///
/// - ratio (ratio): A ratio of any value.
/// - format (str): The wire format being encoded for. Unused -- `ratio`
///   has the same tagged-dictionary shape in every format.
/// -> float
#let encode(ratio, format: "cbor") = {
  if is-debug-assert {
    assert(
      type(ratio) == std.ratio,
      message: "ratio.encode: ratio must be of type ratio",
    )
  }

  (
    typwire-type: type-name,
    ratio: ratio-to-float(ratio),
  )
}

/// Decode a wire-compatible dictionary into a ratio.
///
/// - value (dictionary): A dictionary representing the `encode`d value.
/// - format (str): The wire format being decoded from. Unused -- `ratio`
///   decodes the same way regardless of format.
/// -> ratio
#let decode(value, format: "cbor") = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "ratio.decode: value must be a tagged with type "
        + type-name
        + ". Found "
        + value.typwire-type,
    )
    assert(
      type(value.ratio) == std.float,
      message: "ratio.decode: value.ratio must be of type float, Found "
        + type(value.ratio),
    )
  }

  float-to-ratio(value.ratio)
}
