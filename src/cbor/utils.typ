/// Convert a ratio (0% - 100%) to an integer (0 - 255).
///
/// - ratio (ratio): A ratio value between 0% and 100%.
/// -> int
#let ratio-to-int(ratio) = {
  int(255 * calc.clamp(ratio / 100%, 0, 1))
}

/// Convert a ratio to a float.
///
/// - ratio (ratio): A ratio of any value.
/// -> float
#let ratio-to-float(ratio) = {
  ratio / 100%
}

/// Convert a float to a ratio.
///
/// - float (float): A float value.
/// -> ratio
#let float-to-ratio(float) = {
  float * 100%
}

/// Convert a float to a length.
///
/// - float (float): A float value.
/// -> length
#let float-to-length(float) = {
  float * 1pt
}

/// Check if debug assertions are enabled.
///
/// -> bool
#let is-debug-assert = sys.inputs.at("debug-assert", default: false) == "true"

/// Reverse `repr`'s escaping for the inner content of a quoted string
/// literal (the text between the quotes, not including them).
///
/// Handles every escape `repr` actually emits for a string (confirmed
/// empirically, since it isn't fully spelled out in the docs): `\"`, `\\`,
/// `\n`, `\t`, `\r`, and `\u{...}` (arbitrary codepoint, variable-width hex,
/// used for any other non-printable character). Pure string manipulation --
/// deliberately not `eval`, which would execute the content as Typst code.
///
/// - content (str): The text between a pair of `repr`-produced quotes.
/// -> str
#let unescape-repr-content(content) = {
  content
    .replace(
      regex("\\\\u\\{([0-9a-fA-F]+)\\}"),
      m => str.from-unicode(int(m.captures.at(0), base: 16)),
    )
    .replace("\\n", "\n")
    .replace("\\t", "\t")
    .replace("\\r", "\r")
    .replace("\\\"", "\"")
    .replace("\\\\", "\\")
}

/// Reverse `repr`'s escaping for a full quoted string literal, e.g.
/// `repr("a\"b")`, quotes included.
///
/// -> str
#let unescape-repr-string(quoted) = {
  unescape-repr-content(quoted.slice(1, -1))
}
