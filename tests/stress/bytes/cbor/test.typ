#import "../../../common.typ": plugin, typwire

// A single huge byte payload, tested on its own (separate from the
// multi-argument stress test below it in this same category) since bytes
// need format-specific handling: CBOR keeps them native, while JSON/YAML/
// TOML rewrite them into a tagged array of byte integers (see CLAUDE.md's
// *Wire formats* section) -- the one case whose wire *size*, not just
// shape, meaningfully differs by format under load.
#let count = 40000
#let payload = bytes(range(count).map(i => calc.rem(i, 256)))

#let result = typwire.cbor.call(plugin.stress-bytes-cbor, payload)
#assert.eq(result.len(), count)
#assert.eq(result, payload)
