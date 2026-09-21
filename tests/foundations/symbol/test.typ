#import "../../common.typ": roundtrip, plugin, typwire

// A symbol built from a single string has no modifier variants, so it
// round-trips exactly like any other native type.
#roundtrip(plugin.roundtrip_symbol, symbol("★"))
#roundtrip(plugin.roundtrip_symbol, symbol("é"))

// `sym.arrow.r` (and most of `sym.*`) carries a whole table of named
// modifier variants (`.double`, `.long`, `.squiggly`, ...), but Typst
// exposes no way to enumerate them from script -- only the current glyph
// is observable at all (`str`/`repr`/Typst's own `Serialize` for `Symbol`
// all only expose it). So only that glyph survives the wire; the value
// coming back is a variant-less symbol and is intentionally *not* `==` to
// the original. What's guaranteed is that its current glyph matches.
#let original = sym.arrow.r
#let decoded = typwire.call(plugin.roundtrip_symbol, original)
#assert.eq(str(decoded), str(original))
#assert(decoded != original)
