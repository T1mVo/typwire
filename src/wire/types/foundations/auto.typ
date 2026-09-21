#let type-name = "auto"

/// Encode `auto` into a CBOR-compatible dictionary.
///
/// -> dictionary
#let encode() = {
  (
    typwire-type: type-name,
  )
}

/// Decode a CBOR-compatible dictionary into `auto`.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// -> auto
#let decode(value) = {
  auto
}
