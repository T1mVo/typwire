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

#let known-type(name) = {
  if name == "integer" {
    type(0)
  } else if name == "float" {
    type(0.0)
  } else if name == "bytes" {
    type(bytes())
  } else if name == "string" {
    type("")
  } else if name == "boolean" {
    type(false)
  } else if name == "none" {
    type(none)
  } else if name == "array" {
    type(())
  } else if name == "dictionary" {
    type((:))
  } else if name == "angle" {
    type(0deg)
  } else if name == "length" {
    type(0pt)
  } else if name == "ratio" {
    type(0%)
  } else if name == "color" {
    type(black)
  } else if name == "gradient" {
    type(gradient.linear(black, white))
  } else if name == "datetime" {
    type(datetime(year: 2000))
  } else if name == "duration" {
    type(duration(seconds: 0))
  } else if name == "version" {
    type(version(0, 0, 0))
  } else if name == "type" {
    type(type)
  } else if name == "content" {
    content
  } else if name == "function" {
    function
  } else {
    panic("typwire.decode: cannot reconstruct unknown type object `" + name + "`")
  }
}

#let decode-datetime(value) = {
  let fields = (:)
  for field in ("year", "month", "day", "hour", "minute", "second") {
    if value.at(field) != none {
      fields.insert(field, value.at(field))
    }
  }
  datetime(..fields)
}

#let decode-color(tag, value) = {
  if tag == "color-luma" {
    color.luma(value.lightness, value.alpha)
  } else if tag == "color-oklab" {
    color.oklab(value.lightness, value.a, value.b, value.alpha)
  } else if tag == "color-oklch" {
    color.oklch(value.lightness, value.chroma, value.hue, value.alpha)
  } else if tag == "color-linear-rgb" {
    color.linear-rgb(value.r, value.g, value.b, value.alpha)
  } else if tag == "color-rgb" {
    color.rgb(value.r, value.g, value.b, value.alpha)
  } else if tag == "color-cmyk" {
    color.cmyk(value.cyan, value.magenta, value.yellow, value.key)
  } else if tag == "color-hsl" {
    color.hsl(value.hue, value.saturation, value.lightness, value.alpha)
  } else if tag == "color-hsv" {
    color.hsv(value.hue, value.saturation, value.value, value.alpha)
  } else {
    panic("typwire.decode: unknown typwire-type tag `" + tag + "`")
  }
}

#let decode-gradient(tag, value) = {
  let stops = value.stops.map(stop => (stop.color, stop.offset))
  let space = color-space(value.space)

  if tag == "gradient-linear" {
    gradient.linear(..stops, angle: value.angle, space: space)
  } else if tag == "gradient-radial" {
    gradient.radial(
      ..stops,
      center: (value.center.x, value.center.y),
      radius: value.radius,
      focal-center: (value.focal-center.x, value.focal-center.y),
      focal-radius: value.focal-radius,
      space: space,
    )
  } else if tag == "gradient-conic" {
    gradient.conic(
      ..stops,
      angle: value.angle,
      center: (value.center.x, value.center.y),
      space: space,
    )
  } else {
    panic("typwire.decode: unknown typwire-type tag `" + tag + "`")
  }
}

#let decode-tagged(tag, value) = {
  if tag == "angle" {
    value.radians * 1rad
  } else if tag == "length" {
    value.points * 1pt
  } else if tag == "ratio" {
    value.ratio * 100%
  } else if tag.starts-with("color-") {
    decode-color(tag, value)
  } else if tag.starts-with("gradient-") {
    decode-gradient(tag, value)
  } else if tag == "datetime" {
    decode-datetime(value)
  } else if tag == "duration" {
    let seconds = int(value.seconds)
    if seconds != value.seconds {
      panic("typwire.decode: Typst cannot reconstruct a duration with fractional seconds")
    }
    duration(seconds: seconds)
  } else if tag == "version" {
    version(value.major, value.minor, value.patch, value.revision, value.build)
  } else if tag == "type" {
    known-type(value.ty)
  } else {
    panic("typwire.decode: unknown typwire-type tag `" + tag + "`")
  }
}

#let decode-inner(value) = {
  if type(value) == array {
    value.map(decode-inner)
  } else if type(value) == dictionary {
    let decoded = (:)
    for (key, field) in value.pairs() {
      decoded.insert(key, decode-inner(field))
    }

    if decoded.keys().contains("typwire-type") {
      let tag = decoded.at("typwire-type")
      if type(tag) != str {
        panic("typwire.decode: typwire-type must be a string")
      }
      decode-tagged(tag, decoded)
    } else {
      decoded
    }
  } else {
    value
  }
}

/// Decode CBOR bytes and reconstruct supported native Typst values.
///
/// - data (bytes): The encoded Typwire value.
/// -> any
#let decode(data) = {
  if type(data) != bytes {
    panic("typwire.decode: data must be bytes")
  }
  decode-inner(cbor(data))
}
