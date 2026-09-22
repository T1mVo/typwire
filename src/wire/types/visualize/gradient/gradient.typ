#import "linear.typ"
#import "radial.typ"
#import "conic.typ"
#import "stop.typ"
#import "center.typ"
#import "../../../utils.typ": is-debug-assert

#let common-type-name = "gradient-"

/// Encode a gradient into a wire-compatible dictionary.
///
/// - gradient (gradient): A gradient.
/// - format (str): The wire format being encoded for. Forwarded to every
///   nested `encode` call (stops, center(s), and the kind-specific
///   encoder), though none of them currently use it either.
/// -> dictionary
#let encode(gradient, format: "cbor") = {
  if is-debug-assert {
    assert(
      type(gradient) == std.gradient,
      message: "gradient.encode: gradient must be of type gradient",
    )
  }

  let stops = gradient.stops().map(s => stop.encode(s.at(0), s.at(1), format: format))
  let space = if gradient.space() == std.color.luma {
    "luma"
  } else if gradient.space() == std.color.oklab {
    "oklab"
  } else if gradient.space() == std.color.oklch {
    "oklch"
  } else if gradient.space() == std.color.linear-rgb {
    "linear-rgb"
  } else if gradient.space() == std.color.rgb {
    "rgb"
  } else if gradient.space() == std.color.cmyk {
    "cmyk"
  } else if gradient.space() == std.color.hsl {
    "hsl"
  } else if gradient.space() == std.color.hsv {
    "hsv"
  } else {
    panic("gradient.encode: Invalid gradient color space")
  }

  if gradient.kind() == std.gradient.linear {
    linear.encode(stops, gradient.angle(), space, format: format)
  } else if gradient.kind() == std.gradient.radial {
    radial.encode(
      stops,
      center.encode(..gradient.center(), format: format),
      gradient.radius(),
      center.encode(..gradient.focal-center(), format: format),
      gradient.focal-radius(),
      space,
      format: format,
    )
  } else if gradient.kind() == std.gradient.conic {
    conic.encode(
      stops,
      gradient.angle(),
      center.encode(..gradient.center(), format: format),
      space,
      format: format,
    )
  } else {
    panic("gradient.encode: Invalid gradient type")
  }
}

/// Decode a wire-compatible dictionary into a gradient.
///
/// - tag (string): The typwire-type tag of the encoded gradient.
/// - value (dictionary): A dictionary representing the `encode`d value.
/// - format (str): The wire format being decoded from. Forwarded to the
///   kind-specific `decode`.
/// -> gradient
#let decode(tag, value, format: "cbor") = {
  if tag == linear.type-name {
    linear.decode(value, format: format)
  } else if tag == radial.type-name {
    radial.decode(value, format: format)
  } else if tag == conic.type-name {
    conic.decode(value, format: format)
  } else {
    panic("typwire.decode: unknown typwire-type tag `" + tag + "`")
  }
}
