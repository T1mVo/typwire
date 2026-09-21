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

#let decode-tagged(tag, value) = {
  if tag == alignment.type-name {
    alignment.decode(value)
  } else if tag == angle.type-name {
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
  } else if tag == stroke.type-name {
    stroke.decode(value)
  } else if tag == datetime.type-name {
    datetime.decode(value)
  } else if tag == decimal.type-name {
    decimal.decode(value)
  } else if tag == duration.type-name {
    duration.decode(value)
  } else if tag == label.type-name {
    label.decode(value)
  } else if tag == version.type-name {
    version.decode(value)
  } else if tag == type.type-name {
    type.decode(value)
  } else if tag == path.type-name {
    path.decode(value)
  } else if tag == symbol.type-name {
    symbol.decode(value)
  } else if tag == regex.type-name {
    regex.decode(value)
  } else if tag == arguments.type-name {
    arguments.decode(value)
  } else if tag == "bytes" {
    // Not a native type with its own `types/**` module -- formats with no
    // native byte-string type (everything but CBOR) rewrite a `bytes` leaf
    // into this exact tagged shape before handing the tree to their native
    // encoder; see `wire/rewrite-bytes.typ`. `Cbor` never produces this tag
    // (its own `bytes` values pass straight through untagged), so this arm
    // simply never triggers on the CBOR path.
    bytes(value.data)
  } else {
    panic("typwire.decode: unknown typwire-type tag `" + tag + "`")
  }
}

/// The mirror of `encode.typ::encode`: recursively reconstruct native
/// Typst values from a plain dictionary/array/primitive tree. Format-
/// agnostic -- this is the shared layer every `typwire.<format>.decode`
/// wraps a native format decoder with (`cbor(...)`, `json(...)`, ...).
///
/// - value (any): A tree produced by `encode`, after a native format
///   decoder has parsed it back out of the wire bytes.
/// -> any
#let decode(value) = {
  if std.type(value) == array {
    value.map(decode)
  } else if std.type(value) == dictionary {
    let decoded = (:)
    for (key, field) in value.pairs() {
      decoded.insert(key, decode(field))
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