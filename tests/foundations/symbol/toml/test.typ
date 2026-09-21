#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// A symbol built from a single string has no modifier variants, so it
// round-trips exactly like any other native type.
#roundtrip-format(typwire.toml, exports.at("roundtrip-symbol-toml"), symbol("★"))
#roundtrip-format(typwire.toml, exports.at("roundtrip-symbol-toml"), symbol("é"))

// A symbol constructed with multiple named variants round-trips `==`-exact
// too: `encode` recovers the full variant table from `repr`, and
// reconstructing with `symbol(default, ..variants)` in the same order
// `repr` listed them lands back on an equal value.
#roundtrip-format(typwire.toml, exports.at("roundtrip-symbol-toml"), symbol("a", ("mod1", "b"), ("mod2", "c")))

// `sym.arrow.r` (and most of `sym.*`) carries a whole table of named
// modifier variants (`.double`, `.long`, `.squiggly`, ...). Those *do*
// survive the wire now (unlike a plain `str`-only encoding), but the result
// is still never `==` to a builtin: Typst represents predefined symbols and
// anything `symbol(...)` constructs as different internal representations,
// regardless of how faithfully the variant data is restored. What's
// guaranteed is functional equivalence -- the default glyph and every
// modifier variant resolve to the same text as the original.
#let original = sym.arrow.r
#let decoded = typwire.toml.call(exports.at("roundtrip-symbol-toml"), original)
#assert.eq(str(decoded), str(original))
#assert.eq(str(decoded.double), str(original.double))
#assert.eq(str(decoded.long), str(original.long))
#assert.eq(str(decoded.squiggly), str(original.squiggly))
#assert(decoded != original)
