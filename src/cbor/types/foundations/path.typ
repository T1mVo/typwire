#import "../../utils.typ": is-debug-assert

#let type-name = "path"

/// Undo the escaping `repr` applies to a string literal, without `eval`ing
/// anything -- pure string manipulation, so this can never execute code.
///
/// - value (str): A `repr`'d string, quotes included (e.g. `repr("a\"b")`,
///   i.e. the four characters `"`, `a`, `\`, `"`, `b`, `"`).
/// -> str
#let unescape-repr-string(value) = {
  value
    .slice(1, -1)
    // `\u{...}` is `repr`'s escape for any non-printable character that
    // doesn't have one of the dedicated escapes below (arbitrary hex
    // codepoint, no fixed width, e.g. `\u{7}`, `\u{1b}`).
    .replace(
      regex("\\\\u\\{([0-9a-fA-F]+)\\}"),
      m => str.from-unicode(int(m.captures.at(0), base: 16)),
    )
    .replace("\\n", "\n")
    .replace("\\t", "\t")
    .replace("\\r", "\r")
    .replace("\\\"", "\"")
    // A path can never legitimately contain a backslash (`path(...)`
    // rejects it outright), so `repr` never has an actual backslash to
    // escape here -- this is just defensive completeness, done last so it
    // can't be misread as the start of one of the escapes above.
    .replace("\\\\", "\\")
}

/// Encode a path into a CBOR-compatible dictionary.
///
/// - value (path): A path.
/// -> dictionary
#let encode(value) = {
  if is-debug-assert {
    assert(
      type(value) == path,
      message: "path.encode: value must be of type path",
    )
  }

  // `path` has no built-in string conversion (`str(path(...))` errors, and
  // it exposes no method to read its segments back out) -- the only textual
  // form available at all is `repr()`, which always wraps it as the Typst
  // source expression `path("...")`. Stripping that wrapper leaves the
  // quoted, escaped string literal for the path text, which
  // `unescape-repr-string` then decodes by hand (no `eval`, so this can
  // never run arbitrary code).
  let inner = unescape-repr-string(repr(value).slice(5, -1))

  (
    typwire-type: type-name,
    path: inner,
  )
}

/// Decode a CBOR-compatible dictionary into a path.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// -> path
#let decode(value) = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "path.decode: value must be a tagged with type " + type-name + ". Found " + value.typwire-type,
    )
    assert(
      type(value.path) == str,
      message: "path.decode: value.path must be of type str, Found " + str(type(value.path)),
    )
  }

  path(value.path)
}
