#import "luma.typ"
#import "oklab.typ"
#import "oklch.typ"
#import "linear-rgb.typ"
#import "rgb.typ"
#import "cmyk.typ"
#import "hsl.typ"
#import "hsv.typ"
#import "../../../utils.typ": is-debug-assert

#let common-type-name = "color-"

/// Encode a color into a wire-compatible dictionary.
///
/// - color (color): The color to encode.
/// - format (str): The wire format being encoded for. Forwarded to the
///   per-color-space `encode`, though none of them currently use it either
///   -- every color space has the same tagged-dictionary shape in every
///   format.
/// -> dictionary
#let encode(color, format: "cbor") = {
  if is-debug-assert {
    assert(
      type(color) == std.color,
      message: "color.encode: color must be of type color",
    )
  }

  let c = color.components(alpha: true)

  if color.space() == std.color.luma {
    luma.encode(c.at(0), c.at(1), format: format)
  } else if color.space() == std.color.oklab {
    oklab.encode(c.at(0), c.at(1), c.at(2), c.at(3), format: format)
  } else if color.space() == std.color.oklch {
    oklch.encode(c.at(0), c.at(1), c.at(2), c.at(3), format: format)
  } else if color.space() == std.color.linear-rgb {
    linear-rgb.encode(c.at(0), c.at(1), c.at(2), c.at(3), format: format)
  } else if color.space() == std.color.rgb {
    rgb.encode(c.at(0), c.at(1), c.at(2), c.at(3), format: format)
  } else if color.space() == std.color.cmyk {
    cmyk.encode(c.at(0), c.at(1), c.at(2), c.at(3), format: format)
  } else if color.space() == std.color.hsl {
    hsl.encode(c.at(0), c.at(1), c.at(2), c.at(3), format: format)
  } else if color.space() == std.color.hsv {
    hsv.encode(c.at(0), c.at(1), c.at(2), c.at(3), format: format)
  } else {
    panic("color.encode: Unsupported color type")
  }
}

/// Map a color space name to its Typst color space.
///
/// - name (str): A color space name.
/// -> function
#let color-space(name) = {
  if name == "luma" {
    color.luma
  } else if name == "oklab" {
    color.oklab
  } else if name == "oklch" {
    color.oklch
  } else if name == "linear-rgb" {
    color.linear-rgb
  } else if name == "rgb" {
    color.rgb
  } else if name == "cmyk" {
    color.cmyk
  } else if name == "hsl" {
    color.hsl
  } else if name == "hsv" {
    color.hsv
  } else {
    panic("typwire.decode: unsupported color space `" + name + "`")
  }
}

/// Decode a wire-compatible dictionary into a color.
///
/// - tag (string): The typwire-type tag of the encoded color.
/// - value (dictionary): A dictionary representing the `encode`d value.
/// - format (str): The wire format being decoded from. Forwarded to the
///   per-color-space `decode`, though none of them currently use it either
///   -- every color space decodes the same way regardless of format.
/// -> color
#let decode(tag, value, format: "cbor") = {
  if tag == luma.type-name {
    luma.decode(value, format: format)
  } else if tag == oklab.type-name {
    oklab.decode(value, format: format)
  } else if tag == oklch.type-name {
    oklch.decode(value, format: format)
  } else if tag == linear-rgb.type-name {
    linear-rgb.decode(value, format: format)
  } else if tag == rgb.type-name {
    rgb.decode(value, format: format)
  } else if tag == cmyk.type-name {
    cmyk.decode(value, format: format)
  } else if tag == hsl.type-name {
    hsl.decode(value, format: format)
  } else if tag == hsv.type-name {
    hsv.decode(value, format: format)
  } else {
    panic("typwire.decode: unknown typwire-type tag `" + tag + "`")
  }
}
