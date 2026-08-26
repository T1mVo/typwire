#import "utils.typ": is-debug-assert

#let type-name = "angle"

/// Encode an angle into a CBOR-compatible dictionary.
///
/// - angle (angle): A angle.
/// -> dictionary
#let encode(angle) = {
  if is-debug-assert {
    assert(
      type(angle) == std.angle,
      message: "angle.encode: angle must be of type angle",
    )
  }

  (
    typwire-type: type-name,
    radians: angle.rad(),
  )
}

/// Decode a CBOR-compatible dictionary into an angle.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// -> angle
#let decode(value) = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "angle.decode: value must be a tagged with type "
        + type-name
        + ". Found "
        + value.typwire-type,
    )
    assert(
      type(value.radians) == std.float,
      message: "angle.decode: value.radians must be a float. Found "
        + type(value.radians),
    )
  }

  value.radians * 1rad
}
