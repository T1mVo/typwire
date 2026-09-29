#import "types/layout/alignment.typ"
#import "types/layout/angle.typ"
#import "types/layout/length.typ"
#import "types/layout/ratio.typ"

#import "types/foundations/arguments.typ"

// `auto` is a Typst keyword/literal, not an ordinary identifier -- a bare
// `auto` always evaluates to that literal, so the module must be imported
// under a different name to be reachable at all.
#import "types/foundations/auto.typ" as auto-value
// Imported under a distinct name for the same reason `encode.typ` does --
// see its own comment on this import.
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

/// - tag (str): The `typwire-type` tag read off a dictionary in the tree.
/// - value (dictionary): The dictionary itself (already recursively
///   decoded), with `tag` still present under its `typwire-type` key.
/// - format (str): The wire format being decoded from -- one of `"cbor"`,
///   `"json"`, `"yaml"`, `"toml"`. Forwarded to every per-type `decode`,
///   though only the `"bytes"`/`"none"` arms below actually depend on the
///   tag having been produced by a specific format in the first place --
///   every per-type `decode` ignores it, since a tag alone already fully
///   determines how to decode it, independent of which format the bytes
///   originally came from.
/// -> any
#let decode-tagged(tag, value, format: "cbor") = {
  if tag == alignment.type-name {
    alignment.decode(value, format: format)
  } else if tag == angle.type-name {
    angle.decode(value, format: format)
  } else if tag == auto-value.type-name {
    auto-value.decode(value, format: format)
  } else if tag == length.type-name {
    length.decode(value, format: format)
  } else if tag == ratio.type-name {
    ratio.decode(value, format: format)
  } else if tag.starts-with(color.common-type-name) {
    color.decode(tag, value, format: format)
  } else if tag.starts-with(gradient.common-type-name) {
    gradient.decode(tag, value, format: format)
  } else if tag == stroke.type-name {
    stroke.decode(value, format: format)
  } else if tag == datetime.type-name {
    datetime.decode(value, format: format)
  } else if tag == decimal.type-name {
    decimal.decode(value, format: format)
  } else if tag == duration.type-name {
    duration.decode(value, format: format)
  } else if tag == label.type-name {
    label.decode(value, format: format)
  } else if tag == version.type-name {
    version.decode(value, format: format)
  } else if tag == type.type-name {
    type.decode(value, format: format)
  } else if tag == path.type-name {
    path.decode(value, format: format)
  } else if tag == symbol.type-name {
    symbol.decode(value, format: format)
  } else if tag == regex.type-name {
    regex.decode(value, format: format)
  } else if tag == arguments.type-name {
    arguments.decode(value, format: format)
  } else if tag == bytes-type.type-name {
    // Formats with no native byte-string type (everything but CBOR) tag a
    // `bytes` leaf with this exact shape while `encode.typ::encode` builds
    // the tree, in the same pass as everything else; `Cbor` never produces
    // this tag (its own `bytes` values pass straight through untagged and
    // are caught by `decode`'s own primitive fallback below, never
    // reaching this dispatch at all), so this arm simply never triggers on
    // the CBOR path.
    bytes-type.decode(value, format: format)
  } else if tag == "none" {
    // Same situation as `"bytes"` above, but for `none`/null: only TOML
    // needs this -- `Cbor`/`json`/`yaml` never produce this tag, since
    // they all represent `none` natively. Not its own `types/**` module,
    // unlike `bytes` -- there's nothing to decode beyond the literal.
    none
  } else {
    panic("typwire.decode: unknown typwire-type tag `" + tag + "`")
  }
}

/// The mirror of `encode.typ::encode`: recursively reconstruct native
/// Typst values from a plain dictionary/array/primitive tree. This is the
/// shared layer every `typwire.<format>.decode` wraps a native format
/// decoder with (`cbor(...)`, `json(...)`, ...).
///
/// - value (any): A tree produced by `encode`, after a native format
///   decoder has parsed it back out of the wire bytes.
/// - format (str): The wire format being decoded from -- one of `"cbor"`,
///   `"json"`, `"yaml"`, `"toml"`. Forwarded to `decode-tagged` and every
///   per-type `decode`; see its own doc comment for why almost none of
///   them actually need it.
/// -> any
#let decode(value, format: "cbor") = {
  let value-type = std.type(value)

  if value-type == array {
    value.map(v => decode(v, format: format))
  } else if value-type == dictionary {
    let decoded = (:)
    for (key, field) in value.pairs() {
      decoded.insert(key, decode(field, format: format))
    }

    if "typwire-type" in decoded {
      let tag = decoded.typwire-type

      assert(std.type(tag) == str, message: "typwire.decode: typwire-type must be a string")

      decode-tagged(tag, decoded, format: format)
    } else {
      decoded
    }
  } else {
    value
  }
}
