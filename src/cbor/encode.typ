#import "types/layout/angle.typ"
#import "types/layout/length.typ"
#import "types/layout/ratio.typ"

// `auto` is a Typst keyword/literal, not an ordinary identifier -- a bare
// `auto` always evaluates to that literal, so the module must be imported
// under a different name to be reachable at all.
#import "types/foundations/auto.typ" as auto-value
#import "types/foundations/datetime.typ"
#import "types/foundations/duration.typ"
#import "types/foundations/type.typ"
#import "types/foundations/version.typ"

#import "types/visualize/color/color.typ"
#import "types/visualize/gradient/gradient.typ"

#let is-cbor-compatible(content) = {
  (
    std.type(content) == std.int
      or std.type(content) == std.float
      or std.type(content) == std.bytes
      or std.type(content) == std.str
      or std.type(content) == std.bool
      or content == none
  )
}

#let encode-inner(content) = {
  let result

  if is-cbor-compatible(content) {
    result = content
  } else if std.type(content) == std.array {
    result = content.map(i => encode-inner(i))
  } else if std.type(content) == std.dictionary {
    result = (:)
    for (key, value) in content.pairs() {
      result.insert(key, encode-inner(value))
    }
  } else if content == auto {
    // `auto` has no dedicated `std.type`; unlike `none` it isn't part of
    // Typst's own reliable CBOR subset, so it needs a tagged dict like the
    // other native types below, distinguishing it on the wire from `none`.
    result = encode-inner(auto-value.encode())
  } else if std.type(content) == std.angle {
    result = encode-inner(angle.encode(content))
  } else if std.type(content) == std.length {
    result = encode-inner(length.encode(content))
  } else if std.type(content) == std.ratio {
    result = encode-inner(ratio.encode(content))
  } else if std.type(content) == std.color {
    result = encode-inner(color.encode(content))
  } else if std.type(content) == std.gradient {
    result = encode-inner(gradient.encode(content))
  } else if std.type(content) == std.datetime {
    result = encode-inner(datetime.encode(content))
  } else if std.type(content) == std.duration {
    result = encode-inner(duration.encode(content))
  } else if std.type(content) == std.version {
    result = encode-inner(version.encode(content))
  } else if std.type(content) == std.type {
    result = encode-inner(type.encode(content))
  } else if std.type(content) == std.content {
    panic("typwire.encode: content values are not supported")
  } else {
    panic("typwire.encode: unsupported value type " + str(std.type(content)))
  }

  result
}

/// Encode content into CBOR format.
///
/// - content (any): The content to encode.
/// -> bytes
#let encode(content) = {
  cbor.encode(encode-inner(content))
}
