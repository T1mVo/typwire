#import "../../common.typ": roundtrip-format, plugin, typwire

// Primitives.
#roundtrip-format(typwire.json, plugin.roundtrip-any-json, none)
#roundtrip-format(typwire.json, plugin.roundtrip-any-json, auto)
#roundtrip-format(typwire.json, plugin.roundtrip-any-json, true)
#roundtrip-format(typwire.json, plugin.roundtrip-any-json, 42)
#roundtrip-format(typwire.json, plugin.roundtrip-any-json, 3.5)
#roundtrip-format(typwire.json, plugin.roundtrip-any-json, "text")
#roundtrip-format(typwire.json, plugin.roundtrip-any-json, bytes((1, 2, 3)))
#roundtrip-format(typwire.json, plugin.roundtrip-any-json, path("test.typ"))
#roundtrip-format(typwire.json, plugin.roundtrip-any-json, symbol("★"))
#roundtrip-format(typwire.json, plugin.roundtrip-any-json, regex("a.*b"))
#roundtrip-format(typwire.json, plugin.roundtrip-any-json, decimal("3.14"))
#roundtrip-format(typwire.json, plugin.roundtrip-any-json, arguments(1, "two", a: 3))
#roundtrip-format(typwire.json, plugin.roundtrip-any-json, label("intro"))
#roundtrip-format(typwire.json, plugin.roundtrip-any-json, left + top)
#roundtrip-format(typwire.json, plugin.roundtrip-any-json, stroke(paint: red, thickness: 2pt))

// Nested containers mixing native Typst values.
#roundtrip-format(typwire.json, plugin.roundtrip-any-json,
  (
    source: "any",
    enabled: true,
    count: 3,
    nested: (90deg, 25%, red),
    size: 12pt,
  ),
)
#roundtrip-format(typwire.json, plugin.roundtrip-any-json, (1, "two", 3.0, (nested: true)))
