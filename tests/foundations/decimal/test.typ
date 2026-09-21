#import "../../common.typ": roundtrip, plugin

#roundtrip(plugin.roundtrip_decimal, decimal("0"))
#roundtrip(plugin.roundtrip_decimal, decimal("3.14"))

// Regression test: Typst's `Display` for a negative decimal writes the
// typographic minus sign U+2212, not ASCII `-` -- `rust_decimal` (what the
// Rust side parses with directly) only understands the latter. See the
// comment in `decimal.typ::encode`.
#roundtrip(plugin.roundtrip_decimal, decimal("-3.14"))
#roundtrip(plugin.roundtrip_decimal, decimal("-0.5"))

// High precision, near `rust_decimal`'s digit limit.
#roundtrip(plugin.roundtrip_decimal, decimal("79228162514264337593543950335"))
#roundtrip(plugin.roundtrip_decimal, decimal("-79228162514264337593543950335"))
