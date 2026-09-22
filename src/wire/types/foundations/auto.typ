#let type-name = "auto"

/// Encode `auto` into a wire-compatible dictionary.
///
/// - format (str): The wire format being encoded for. Unused -- `auto` has
///   the same tagged-dictionary shape in every format.
/// -> dictionary
#let encode(format: "cbor") = {
  (
    typwire-type: type-name,
  )
}

/// Decode a wire-compatible dictionary into `auto`.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// - format (str): The wire format being decoded from. Unused -- `auto`
///   decodes the same way regardless of format.
/// -> auto
#let decode(value, format: "cbor") = {
  auto
}
