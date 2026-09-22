#import "../../utils.typ": is-debug-assert

#let type-name = "angle"

/// Encode an angle into a wire-compatible dictionary.
///
/// - angle (angle): A angle.
/// - format (str): The wire format being encoded for. Unused -- `angle`
///   has the same tagged-dictionary shape in every format.
/// -> dictionary
#let encode(angle, format: "cbor") = {
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

/// Decode a wire-compatible dictionary into an angle.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// - format (str): The wire format being decoded from. Unused -- `angle`
///   decodes the same way regardless of format.
/// -> angle
#let decode(value, format: "cbor") = {
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
