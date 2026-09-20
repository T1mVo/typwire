#import "../../utils.typ": is-debug-assert
#import "../color/color.typ"
#import "center.typ"
#import "stop.typ"

#let type-name = "gradient-conic"

/// Encode a conic gradient into a CBOR-compatible dictionary.
///
/// - stops (array): An array of encoded stops.
/// - angle (angle): The angle of the gradient.
/// - center (dictionary): An encoded center.
/// - space (str): The name of the color space.
/// -> dictionary
#let encode(stops, angle, center, space) = {
  if is-debug-assert {
    assert(
      type(stops) == std.array,
      message: "conic.encode: stops must be of type array",
    )
    assert(
      type(angle) == std.angle,
      message: "conic.encode: angle must be of type angle",
    )
    assert(
      type(center) == std.dictionary,
      message: "conic.encode: center must be of type dictionary",
    )
    assert(
      type(space) == std.str,
      message: "conic.encode: space must be of type str",
    )
  }

  (
    typwire-type: type-name,
    stops: stops,
    angle: angle,
    center: center,
    space: space,
  )
}

/// Decode a CBOR-compatible dictionary into a conic gradient.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// -> gradient
#let decode(value) = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "conic.decode: value must be a tagged with type "
        + type-name
        + ". Found "
        + value.typwire-type,
    )
    assert(
      type(value.stops) == std.array,
      message: "conic.decode: value.stops must be of type array, Found "
        + str(type(value.stops)),
    )
    assert(
      type(value.angle) == std.angle,
      message: "conic.decode: value.angle must be of type angle, Found "
        + str(type(value.angle)),
    )
    assert(
      type(value.center) == std.dictionary,
      message: "conic.decode: value.center must be of type dictionary, Found "
        + str(type(value.center)),
    )
    assert(
      type(value.space) == std.str,
      message: "conic.decode: value.space must be of type str, Found "
        + str(type(value.space)),
    )
  }

  let stops = value.stops.map(stop.decode)
  let space = color.color-space(value.space)

  gradient.conic(
    ..stops,
    angle: value.angle,
    center: center.decode(value.center),
    space: space,
  )
}
