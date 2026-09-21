#import "linear.typ"
#import "radial.typ"
#import "conic.typ"
#import "stop.typ"
#import "center.typ"
#import "../../../utils.typ": is-debug-assert

#let common-type-name = "gradient-"

/// Encode a gradient into a CBOR-compatible dictionary.
///
/// - gradient (gradient): A gradient.
/// -> dictionary
#let encode(gradient) = {
  if is-debug-assert {
    assert(
      type(gradient) == std.gradient,
      message: "gradient.encode: gradient must be of type gradient",
    )
  }

  let stops = gradient.stops().map(s => stop.encode(s.at(0), s.at(1)))
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
    linear.encode(stops, gradient.angle(), space)
  } else if gradient.kind() == std.gradient.radial {
    radial.encode(
      stops,
      center.encode(..gradient.center()),
      gradient.radius(),
      center.encode(..gradient.focal-center()),
      gradient.focal-radius(),
      space,
    )
  } else if gradient.kind() == std.gradient.conic {
    conic.encode(
      stops,
      gradient.angle(),
      center.encode(..gradient.center()),
      space,
    )
  } else {
    panic("gradient.encode: Invalid gradient type")
  }
}

/// Decode a CBOR-compatible dictionary into a gradient.
///
/// - tag (string): The typwire-type tag of the encoded gradient.
/// - value (dictionary): A dictionary representing the `encode`d value.
/// -> gradient
#let decode(tag, value) = {
  if tag == linear.type-name {
    linear.decode(value)
  } else if tag == radial.type-name {
    radial.decode(value)
  } else if tag == conic.type-name {
    conic.decode(value)
  } else {
    panic("typwire.decode: unknown typwire-type tag `" + tag + "`")
  }
}
