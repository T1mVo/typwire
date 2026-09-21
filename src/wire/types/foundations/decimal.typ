#import "../../utils.typ": is-debug-assert

#let type-name = "decimal"

/// Encode a decimal into a CBOR-compatible dictionary.
///
/// - value (decimal): A decimal.
/// -> dictionary
#let encode(value) = {
  if is-debug-assert {
    assert(
      type(value) == decimal,
      message: "decimal.encode: value must be of type decimal",
    )
  }

  // Unlike `path`/`symbol`/`regex`, `decimal` has a real `str()` conversion
  // that gives back its exact digits -- no `repr`/escaping trick needed.
  // The one wrinkle: Typst's `Display` for a negative decimal writes the
  // typographic minus sign U+2212, not ASCII `-`. The Rust side parses this
  // string directly with `rust_decimal` (not through Typst), which only
  // understands the latter, so normalize it away here.
  (
    typwire-type: type-name,
    decimal: str(value).replace("\u{2212}", "-"),
  )
}

/// Decode a CBOR-compatible dictionary into a decimal.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// -> decimal
#let decode(value) = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "decimal.decode: value must be a tagged with type " + type-name + ". Found " + value.typwire-type,
    )
    assert(
      type(value.decimal) == str,
      message: "decimal.decode: value.decimal must be of type str, Found " + str(type(value.decimal)),
    )
  }

  decimal(value.decimal)
}
