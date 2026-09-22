#import "types/layout/alignment.typ"
#import "types/layout/angle.typ"
#import "types/layout/length.typ"
#import "types/layout/ratio.typ"

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

#import "types/visualize/color/color.typ"
#import "types/visualize/gradient/gradient.typ"
#import "types/visualize/stroke.typ"

/// Whether `content` is representable as itself on the wire, with no
/// `typwire-type` tagging, under `format`.
///
/// Generalizes what used to be an implicitly CBOR-only check: `int`/
/// `float`/`str`/`bool` are primitive in every format, but `none` is not in
/// every format -- only TOML has no `none`/null at all (every other format
/// needs it tagged, see the `content == none` branch below). `bytes` isn't
/// primitive-or-not here at all anymore -- `bytes-type.encode` below
/// decides its own shape per format directly, the same way every other
/// tagged type's own `encode` does.
///
/// - content (any): The content to check.
/// - format (str): The wire format being encoded for -- one of `"cbor"`,
///   `"json"`, `"yaml"`, `"toml"`.
/// -> bool
#let is-primitive(content, format: "cbor") = {
  (
    std.type(content) == std.int
      or std.type(content) == std.float
      or std.type(content) == std.str
      or std.type(content) == std.bool
      or (content == none and format != "toml")
  )
}

/// Recursively turn a Typst value into a plain dictionary/array/primitive
/// tree: native types become `typwire-type`-tagged dictionaries, everything
/// else passes through unchanged. This is the shared layer every
/// `typwire.<format>.encode` wraps with a native format encoder
/// (`cbor.encode`, `json.encode`, ...) -- `format` is threaded through so
/// the one format-sensitive case (`bytes`, and TOML's `none`) can be
/// resolved in this single pass, rather than a separate tree-rewrite pass
/// afterward.
///
/// - content (any): The content to encode.
/// - format (str): The wire format being encoded for -- one of `"cbor"`,
///   `"json"`, `"yaml"`, `"toml"`.
/// -> any
#let encode(content, format: "cbor") = {
  let result

  if is-primitive(content, format: format) {
    result = content
  } else if std.type(content) == std.bytes {
    // Not wrapped in a recursive `encode(...)` call like every other
    // branch below -- `bytes-type.encode` returns the *same* `bytes` value
    // unchanged under `"cbor"`, so recursing into `encode` on that result
    // would hit this exact branch again and loop forever. Every other
    // type's own `encode` always returns a *new* dictionary shape instead,
    // which is what makes recursing into it there safe and necessary (to
    // walk that dictionary's own fields).
    result = bytes-type.encode(content, format: format)
  } else if content == none {
    // Only reached when `format == "toml"` -- every other format's `none`
    // is already primitive above.
    result = (typwire-type: "none")
  } else if std.type(content) == std.array {
    result = content.map(i => encode(i, format: format))
  } else if std.type(content) == std.dictionary {
    result = (:)
    for (key, value) in content.pairs() {
      result.insert(key, encode(value, format: format))
    }
  } else if content == auto {
    // `auto` has no dedicated `std.type`; unlike `none` it isn't part of
    // Typst's own reliable CBOR subset, so it needs a tagged dict like the
    // other native types below, distinguishing it on the wire from `none`.
    result = encode(auto-value.encode(format: format), format: format)
  } else if std.type(content) == std.alignment {
    result = encode(alignment.encode(content, format: format), format: format)
  } else if std.type(content) == std.angle {
    result = encode(angle.encode(content, format: format), format: format)
  } else if std.type(content) == std.length {
    result = encode(length.encode(content, format: format), format: format)
  } else if std.type(content) == std.ratio {
    result = encode(ratio.encode(content, format: format), format: format)
  } else if std.type(content) == std.color {
    result = encode(color.encode(content, format: format), format: format)
  } else if std.type(content) == std.gradient {
    result = encode(gradient.encode(content, format: format), format: format)
  } else if std.type(content) == std.stroke {
    result = encode(stroke.encode(content, format: format), format: format)
  } else if std.type(content) == std.datetime {
    result = encode(datetime.encode(content, format: format), format: format)
  } else if std.type(content) == std.decimal {
    result = encode(decimal.encode(content, format: format), format: format)
  } else if std.type(content) == std.duration {
    result = encode(duration.encode(content, format: format), format: format)
  } else if std.type(content) == std.label {
    result = encode(label.encode(content, format: format), format: format)
  } else if std.type(content) == std.version {
    result = encode(version.encode(content, format: format), format: format)
  } else if std.type(content) == std.type {
    result = encode(type.encode(content, format: format), format: format)
  } else if std.type(content) == std.path {
    result = encode(path.encode(content, format: format), format: format)
  } else if std.type(content) == std.symbol {
    result = encode(symbol.encode(content, format: format), format: format)
  } else if std.type(content) == std.regex {
    result = encode(regex.encode(content, format: format), format: format)
  } else if std.type(content) == std.arguments {
    result = encode(arguments.encode(content, format: format), format: format)
  } else if std.type(content) == std.content {
    panic("typwire.encode: content values are not supported")
  } else {
    panic("typwire.encode: unsupported value type " + str(std.type(content)))
  }

  result
}
