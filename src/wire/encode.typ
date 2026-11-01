#import "types/foundations/arguments.typ"

// `auto` is a Typst keyword/literal, not an ordinary identifier -- a bare
// `auto` always evaluates to that literal, so the module must be imported
// under a different name to be reachable at all.
#import "types/foundations/auto.typ" as auto-value
// Imported under a distinct name for the same reason `path.typ`/`type.typ`
// already need one at their own call sites (`std.path`/`std.type`): the
// bare name `bytes` also names the Typst builtin's own type/constructor,
// and this file needs both (`std.type(content) == std.bytes` alongside
// `bytes-type.encode(...)`).
#import "types/foundations/bytes.typ" as bytes-type
#import "types/foundations/datetime.typ"
#import "types/foundations/decimal.typ"
#import "types/foundations/duration.typ"
#import "types/foundations/label.typ"
#import "types/foundations/path.typ"
#import "types/foundations/regex.typ"
#import "types/foundations/symbol.typ"
#import "types/foundations/type.typ"
#import "types/foundations/version.typ"
#import "types/layout/alignment.typ"
#import "types/layout/angle.typ"
#import "types/layout/length.typ"
#import "types/layout/ratio.typ"
#import "types/visualize/color/color.typ"
#import "types/visualize/gradient/gradient.typ"
#import "types/visualize/stroke.typ"

/// Whether `content` is representable as itself on the wire, with no
/// `typwire-type` tagging: `int`/`float`/`str`/`bool`/`none` are primitive
/// in both `"cbor"` and `"json"`, the only two formats this crate
/// supports. `bytes` isn't primitive-or-not here at all -- `bytes-type.
/// encode` below decides its own shape per format directly, the same way
/// every other tagged type's own `encode` does.
/// -> bool
#let is-primitive(
  /// `std.type(content)`, already computed by the caller -- accepted
  /// rather than recomputed here since `encode` (this function's only
  /// call site) always already has it.
  /// -> type
  content-type,
  /// The content to check.
  /// -> any
  content,
) = {
  (
    content-type == std.int
      or content-type == std.float
      or content-type == std.str
      or content-type == std.bool
      or content == none
  )
}

/// Recursively turn a Typst value into a plain dictionary/array/primitive
/// tree: native types become `typwire-type`-tagged dictionaries, everything
/// else passes through unchanged. This is the shared layer every
/// `typwire.<format>.encode` wraps with a native format encoder
/// (`cbor.encode`, `json.encode`) -- `format` is threaded through so the
/// one format-sensitive case (`bytes`) can be resolved in this single
/// pass, rather than a separate tree-rewrite pass afterward.
/// -> any
#let encode(
  /// The content to encode.
  /// -> any
  content,
  /// The wire format being encoded for -- one of `"cbor"`, `"json"`.
  /// -> str
  format: "cbor",
) = {
  let content-type = std.type(content)
  let result

  if is-primitive(content-type, content) {
    result = content
  } else if content-type == std.bytes {
    // Not wrapped in a recursive `encode(...)` call like every other
    // branch below -- `bytes-type.encode` returns the *same* `bytes` value
    // unchanged under `"cbor"`, so recursing into `encode` on that result
    // would hit this exact branch again and loop forever. Every other
    // type's own `encode` always returns a *new* dictionary shape instead,
    // which is what makes recursing into it there safe and necessary (to
    // walk that dictionary's own fields).
    result = bytes-type.encode(content, format: format)
  } else if content-type == std.array {
    result = content.map(i => encode(i, format: format))
  } else if content-type == std.dictionary {
    result = (:)
    for (key, value) in content.pairs() {
      result.insert(key, encode(value, format: format))
    }
  } else if content == auto {
    // `auto` has no dedicated `std.type`; unlike `none` it isn't part of
    // Typst's own reliable CBOR subset, so it needs a tagged dict like the
    // other native types below, distinguishing it on the wire from `none`.
    result = encode(auto-value.encode(format: format), format: format)
  } else if content-type == std.alignment {
    result = encode(alignment.encode(content, format: format), format: format)
  } else if content-type == std.angle {
    result = encode(angle.encode(content, format: format), format: format)
  } else if content-type == std.length {
    result = encode(length.encode(content, format: format), format: format)
  } else if content-type == std.ratio {
    result = encode(ratio.encode(content, format: format), format: format)
  } else if content-type == std.color {
    result = encode(color.encode(content, format: format), format: format)
  } else if content-type == std.gradient {
    result = encode(gradient.encode(content, format: format), format: format)
  } else if content-type == std.stroke {
    result = encode(stroke.encode(content, format: format), format: format)
  } else if content-type == std.datetime {
    result = encode(datetime.encode(content, format: format), format: format)
  } else if content-type == std.decimal {
    result = encode(decimal.encode(content, format: format), format: format)
  } else if content-type == std.duration {
    result = encode(duration.encode(content, format: format), format: format)
  } else if content-type == std.label {
    result = encode(label.encode(content, format: format), format: format)
  } else if content-type == std.version {
    result = encode(version.encode(content, format: format), format: format)
  } else if content-type == std.type {
    result = encode(type.encode(content, format: format), format: format)
  } else if content-type == std.path {
    result = encode(path.encode(content, format: format), format: format)
  } else if content-type == std.symbol {
    result = encode(symbol.encode(content, format: format), format: format)
  } else if content-type == std.regex {
    result = encode(regex.encode(content, format: format), format: format)
  } else if content-type == std.arguments {
    result = encode(arguments.encode(content, format: format), format: format)
  } else if content-type == std.content {
    panic("typwire.encode: content values are not supported")
  } else {
    panic("typwire.encode: unsupported value type " + str(content-type))
  }

  result
}
