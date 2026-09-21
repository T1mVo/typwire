#import "../../utils.typ": is-debug-assert, unescape-repr-string

#let type-name = "path"

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
