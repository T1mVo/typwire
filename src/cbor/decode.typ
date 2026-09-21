#import "types/angle.typ"
// `auto` is a Typst keyword/literal, not an ordinary identifier -- a bare
// `auto` always evaluates to that literal, so the module must be imported
// under a different name to be reachable at all.
#import "types/auto.typ" as auto-value
#import "types/color/color.typ"
#import "types/datetime.typ"
#import "types/duration.typ"
#import "types/gradient/gradient.typ"
#import "types/length.typ"
#import "types/ratio.typ"
#import "types/type.typ"
#import "types/version.typ"

#let decode-tagged(tag, value) = {
  if tag == angle.type-name {
    angle.decode(value)
  } else if tag == auto-value.type-name {
    auto-value.decode(value)
  } else if tag == length.type-name {
    length.decode(value)
  } else if tag == ratio.type-name {
    ratio.decode(value)
  } else if tag.starts-with(color.common-type-name) {
    color.decode(tag, value)
  } else if tag.starts-with(gradient.common-type-name) {
    gradient.decode(tag, value)
  } else if tag == datetime.type-name {
    datetime.decode(value)
  } else if tag == duration.type-name {
    duration.decode(value)
  } else if tag == version.type-name {
    version.decode(value)
  } else if tag == type.type-name {
    type.decode(value)
  } else {
    panic("typwire.decode: unknown typwire-type tag `" + tag + "`")
  }
}

#let decode-inner(value) = {
  if std.type(value) == array {
    value.map(decode-inner)
  } else if std.type(value) == dictionary {
    let decoded = (:)
    for (key, field) in value.pairs() {
      decoded.insert(key, decode-inner(field))
    }

    if decoded.keys().contains("typwire-type") {
      let tag = decoded.at("typwire-type")
      if std.type(tag) != str {
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
  if std.type(data) != bytes {
    panic("typwire.decode: data must be bytes")
  }
  decode-inner(cbor(data))
}