#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-decimal-json, decimal("0"))
#roundtrip-format(typwire.json, plugin.roundtrip-decimal-json, decimal("3.14"))

// Regression test: Typst's `Display` for a negative decimal writes the
// typographic minus sign U+2212, not ASCII `-` -- `rust_decimal` (what the
// Rust side parses with directly) only understands the latter. See the
// comment in `decimal.typ::encode`.
#roundtrip-format(typwire.json, plugin.roundtrip-decimal-json, decimal("-3.14"))
#roundtrip-format(typwire.json, plugin.roundtrip-decimal-json, decimal("-0.5"))

// High precision, near `rust_decimal`'s digit limit.
#roundtrip-format(typwire.json, plugin.roundtrip-decimal-json, decimal("79228162514264337593543950335"))
#roundtrip-format(typwire.json, plugin.roundtrip-decimal-json, decimal("-79228162514264337593543950335"))
