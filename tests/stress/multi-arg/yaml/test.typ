#import "../../../common.typ": plugin, typwire

// Multiple arguments, each individually heavy: a long string, a gradient
// with many stops, and a large numeric array -- stressing `typwire.call`'s
// "encode every positional argument independently, decode one combined
// result" convention under load, not just a single big value (see the
// dedicated `stress/bytes` test for that).
#let text = "typst " * 20000
#let stops = gradient.linear(..range(96).map(i => rgb(calc.rem(i * 4, 256), 0, 255 - calc.rem(i * 4, 256))))
#let numbers = range(6000)

#let (out-text, out-stops, out-numbers) = typwire.yaml.call(
  plugin.stress-multi-arg-yaml,
  text,
  stops,
  numbers,
)

#assert.eq(out-text, text)
// Gradients don't compare structurally (see `common.typ`'s `roundtrip`
// doc comment), so this compares by `repr` instead of `==`.
#assert.eq(repr(out-stops), repr(stops))
#assert.eq(out-numbers, numbers)
