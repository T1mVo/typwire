#import "../utils.typ": is-debug-assert
#import "../color/color.typ"
#import "../stop.typ"

#let type-name = "gradient-linear"

/// Encode a linear gradient into a CBOR-compatible dictionary.
///
/// - stops (array): An array of encoded stops.
/// - angle (angle): The angle of the gradient.
/// - space (str): The name of the color space.
/// -> dictionary
#let encode(stops, angle, space) = {
  if is-debug-assert {
    assert(
      type(stops) == std.array,
      message: "linear.encode: stops must be of type array",
    )
    assert(
      type(angle) == std.angle,
      message: "linear.encode: angle must be of type angle",
    )
    assert(
      type(space) == std.str,
      message: "linear.encode: space must be of type str",
    )
  }

  (
    typwire-type: type-name,
    stops: stops,
    angle: angle,
    space: space,
  )
}

/// Decode a CBOR-compatible dictionary into a linear gradient.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// -> gradient
#let decode(value) = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "linear.decode: value must be a tagged with type "
        + type-name
        + ". Found "
        + value.typwire-type,
    )
    assert(
      type(value.stops) == std.array,
      message: "linear.decode: value.stops must be of type array, Found "
        + str(type(value.stops)),
    )
    assert(
      type(value.angle) == std.angle,
      message: "linear.decode: value.angle must be of type angle, Found "
        + str(type(value.angle)),
    )
    assert(
      type(value.space) == std.str,
      message: "linear.decode: value.space must be of type str, Found "
        + str(type(value.space)),
    )
  }

  let stops = value.stops.map(stop.decode)
  let space = color.color-space(value.space)

  gradient.linear(..stops, angle: value.angle, space: space)
}
