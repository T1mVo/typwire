#import "../../utils.typ": is-debug-assert

#let type-name = "version"

/// The number of components `value` was actually constructed with.
///
/// `version` has no accessor for this -- `.at(n)` happily returns `0` for
/// any `n` past the components a value was built with (confirmed
/// empirically: `version(1, 2).at(4)` is `0`, not an error), so reading
/// every value back out via `.at(0)..at(4)` the way this module used to
/// would silently pad every version out to a fixed 5 components instead of
/// preserving how many were actually given. `repr()` is the one place that
/// distinguishes them: `repr(version(1, 2))` is the literal source
/// `"version(1, 2)"`, so counting `", "`-separated entries inside the
/// parens (after stripping the `"version("`/`")"` wrapper) gives the real
/// count; `.at(i)` for `i` in that range then reads back the real
/// components, not text parsed out of `repr`.
#let component-count(value) = {
  let inner = repr(value).slice(8, -1)
  if inner == "" {
    0
  } else {
    inner.split(", ").len()
  }
}

/// Encode a version into a wire-compatible dictionary.
///
/// - version (version): A version.
/// - format (str): The wire format being encoded for. Unused -- `version`
///   has the same tagged-dictionary shape in every format.
/// -> dictionary
#let encode(version, format: "cbor") = {
  if is-debug-assert {
    assert(
      type(version) == std.version,
      message: "version.encode: version must be of type version",
    )
  }

  (
    typwire-type: type-name,
    components: range(component-count(version)).map(i => version.at(i)),
  )
}

/// Decode a wire-compatible dictionary into a version.
///
/// - value (dictionary): A dictionary representing the `encode`d value.
/// - format (str): The wire format being decoded from. Unused -- `version`
///   decodes the same way regardless of format.
/// -> version
#let decode(value, format: "cbor") = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "version.decode: value must be a tagged with type " + type-name + ". Found " + value.typwire-type,
    )

    assert(
      type(value.components) == array,
      message: "version.decode: value.components must be of type array, Found " + str(type(value.components)),
    )
  }

  version(..value.components)
}
