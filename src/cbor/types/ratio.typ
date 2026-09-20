#import "../utils.typ": float-to-ratio, is-debug-assert, ratio-to-float

#let type-name = "ratio"

/// Encode a ratio into a CBOR-compatible float.
///
/// - ratio (ratio): A ratio of any value.
/// -> float
#let encode(ratio) = {
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

/// Decode a CBOR-compatible dictionary into a ratio.
///
/// - value (dictionary): A dictionary representing the `encode`d value.
/// -> ratio
#let decode(value) = {
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
