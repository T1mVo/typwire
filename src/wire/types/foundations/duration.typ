#let type-name = "duration"

#let new(seconds) = {
  assert(type(seconds) == std.float, message: "duration.new: seconds must be of type float")

  (
    "typwire-type": type-name,
    "seconds": seconds,
  )
}

/// Encode a duration into a wire-compatible dictionary.
///
/// - duration (duration): A duration.
/// - format (str): The wire format being encoded for. Unused -- `duration`
///   has the same tagged-dictionary shape in every format.
/// -> dictionary
#let encode(duration, format: "cbor") = {
  assert(type(duration) == std.duration, message: "duration.encode: duration must be type of duration")

  new(duration.seconds())
}

/// Decode a wire-compatible dictionary into a duration.
///
/// - value (dictionary): A dictionary representing the `encode`d duration.
/// - format (str): The wire format being decoded from. Unused --
///   `duration` decodes the same way regardless of format.
/// -> duration
#let decode(value, format: "cbor") = {
  let seconds = int(value.seconds)
  if seconds != value.seconds {
    panic("typwire.decode: Typst cannot reconstruct a duration with fractional seconds")
  }
  duration(seconds: seconds)
}
