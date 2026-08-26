#let new(seconds) = {
  assert(type(seconds) == std.float, message: "duration.new: seconds must be of type float")
  
  (
    "typwire-type": "duration",
    "seconds": seconds,
  )
}

/// Encode a duration into a CBOR-compatible dictionary.
///
/// - duration (duration): A duration.
/// -> dictionary
#let encode(duration) = {
  assert(type(duration) == std.duration, message: "duration.encode: duration must be type of duration")

  new(duration.seconds())
}

/// Decode a CBOR-compatible dictionary into a duration.
///
/// - value (dictionary): A dictionary representing the `encode`d duration.
/// -> duration
#let decode(value) = {
  let seconds = int(value.seconds)
  if seconds != value.seconds {
    panic("typwire.decode: Typst cannot reconstruct a duration with fractional seconds")
  }
  duration(seconds: seconds)
}
