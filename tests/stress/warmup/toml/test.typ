#import "../../../common.typ": plugin, typwire

// Calls the plugin many times with a *different* gradient each time, rather
// than the one-heavy-payload shape every other `stress/` case uses --
// confirmed the hard way that Typst memoizes a plugin call for identical
// arguments (a loop reusing the same value round-tripped ~6x faster than an
// otherwise-identical loop varying it, at 100,000 calls), so a fixed value
// here would measure a cache lookup, not a real call. A gradient built from
// a real color map (`color.map.viridis`), not a bare scalar, since per-call
// overhead scales with payload shape.
//
// Compile with `typst compile --timings out.json
// tests/stress/warmup/toml/test.typ` and look at the `func call` spans in
// `out.json`: the first several calls cost far more than the rest, most
// likely the plugin's wasm linear memory growing in a few steps as its
// allocator needs more heap -- not something that keeps recurring once it
// has grown enough.
#let count = 500
// An integer-degree angle, not a fractional one (e.g. not `calc.rem(i,
// 360) / 100 * 1deg`): confirmed the hard way that a fractional angle
// round-trips exactly through CBOR's binary float encoding but can land on
// a different last decimal digit through JSON's text-based one (`0.055deg`
// came back as `0.06deg` through JSON, `0.05deg` through CBOR, for the
// same source value) -- not a bug in typwire, just not this test's problem
// to have either. Typst doesn't normalize an angle beyond 360 degrees
// (confirmed: `repr(361deg)` is `"361deg"`, not `"1deg"`), so a modulus far
// larger than `count` is all that's needed to keep every value distinct.
#let make-gradient(i) = gradient.linear(..color.map.viridis, angle: calc.rem(i, 100000) * 1deg)

#let values = range(count).map(make-gradient)
#let results = values.map(value => typwire.toml.call(plugin.stress-warmup-toml, value))

// Gradients don't compare structurally (see `common.typ`'s `roundtrip` doc
// comment), so this compares by `repr` instead of `==`.
#for (value, result) in values.zip(results) {
  assert.eq(repr(result), repr(value))
}
