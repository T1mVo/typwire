#import "types/layout/alignment.typ"
#import "types/layout/angle.typ"
#import "types/layout/length.typ"
#import "types/layout/ratio.typ"

#import "types/foundations/arguments.typ"

// `auto` is a Typst keyword/literal, not an ordinary identifier -- a bare
// `auto` always evaluates to that literal, so the module must be imported
// under a different name to be reachable at all.
#import "types/foundations/auto.typ" as auto-value
#import "types/foundations/datetime.typ"
#import "types/foundations/decimal.typ"
#import "types/foundations/duration.typ"
#import "types/foundations/label.typ"
#import "types/foundations/path.typ"
#import "types/foundations/regex.typ"
#import "types/foundations/symbol.typ"
#import "types/foundations/type.typ"
#import "types/foundations/version.typ"

#import "types/visualize/color/color.typ"
#import "types/visualize/gradient/gradient.typ"
#import "types/visualize/stroke.typ"

#let is-primitive(content) = {
  (
    std.type(content) == std.int
      or std.type(content) == std.float
      or std.type(content) == std.bytes
      or std.type(content) == std.str
      or std.type(content) == std.bool
      or content == none
  )
}

/// Recursively turn a Typst value into a plain dictionary/array/primitive
/// tree: native types become `typwire-type`-tagged dictionaries, everything
/// else passes through unchanged. Format-agnostic -- this is the shared
/// layer every `typwire.<format>.encode` wraps with a native format encoder
/// (`cbor.encode`, `json.encode`, ...). `bytes` values pass straight
/// through as real Typst `bytes`; formats with no native byte-string type
/// (everything but CBOR) rewrite them separately -- see
/// `wire/rewrite-bytes.typ`.
///
/// - content (any): The content to encode.
/// -> any
#let encode(content) = {
  let result

  if is-primitive(content) {
    result = content
  } else if std.type(content) == std.array {
    result = content.map(i => encode(i))
  } else if std.type(content) == std.dictionary {
    result = (:)
    for (key, value) in content.pairs() {
      result.insert(key, encode(value))
    }
  } else if content == auto {
    // `auto` has no dedicated `std.type`; unlike `none` it isn't part of
    // Typst's own reliable CBOR subset, so it needs a tagged dict like the
    // other native types below, distinguishing it on the wire from `none`.
    result = encode(auto-value.encode())
  } else if std.type(content) == std.alignment {
    result = encode(alignment.encode(content))
  } else if std.type(content) == std.angle {
    result = encode(angle.encode(content))
  } else if std.type(content) == std.length {
    result = encode(length.encode(content))
  } else if std.type(content) == std.ratio {
    result = encode(ratio.encode(content))
  } else if std.type(content) == std.color {
    result = encode(color.encode(content))
  } else if std.type(content) == std.gradient {
    result = encode(gradient.encode(content))
  } else if std.type(content) == std.stroke {
    result = encode(stroke.encode(content))
  } else if std.type(content) == std.datetime {
    result = encode(datetime.encode(content))
  } else if std.type(content) == std.decimal {
    result = encode(decimal.encode(content))
  } else if std.type(content) == std.duration {
    result = encode(duration.encode(content))
  } else if std.type(content) == std.label {
    result = encode(label.encode(content))
  } else if std.type(content) == std.version {
    result = encode(version.encode(content))
  } else if std.type(content) == std.type {
    result = encode(type.encode(content))
  } else if std.type(content) == std.path {
    result = encode(path.encode(content))
  } else if std.type(content) == std.symbol {
    result = encode(symbol.encode(content))
  } else if std.type(content) == std.regex {
    result = encode(regex.encode(content))
  } else if std.type(content) == std.arguments {
    result = encode(arguments.encode(content))
  } else if std.type(content) == std.content {
    panic("typwire.encode: content values are not supported")
  } else {
    panic("typwire.encode: unsupported value type " + str(std.type(content)))
  }

  result
}
