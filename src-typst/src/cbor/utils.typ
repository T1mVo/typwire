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
