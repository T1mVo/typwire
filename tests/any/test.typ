#import "../common.typ": roundtrip, plugin

// Primitives.
#roundtrip(plugin.roundtrip_any, none)
#roundtrip(plugin.roundtrip_any, auto)
#roundtrip(plugin.roundtrip_any, true)
#roundtrip(plugin.roundtrip_any, 42)
#roundtrip(plugin.roundtrip_any, 3.5)
#roundtrip(plugin.roundtrip_any, "text")
#roundtrip(plugin.roundtrip_any, bytes((1, 2, 3)))
#roundtrip(plugin.roundtrip_any, path("test.typ"))
#roundtrip(plugin.roundtrip_any, symbol("★"))
#roundtrip(plugin.roundtrip_any, regex("a.*b"))
#roundtrip(plugin.roundtrip_any, decimal("3.14"))
#roundtrip(plugin.roundtrip_any, arguments(1, "two", a: 3))
#roundtrip(plugin.roundtrip_any, label("intro"))
#roundtrip(plugin.roundtrip_any, left + top)
#roundtrip(plugin.roundtrip_any, stroke(paint: red, thickness: 2pt))

// Nested containers mixing native Typst values.
#roundtrip(
  plugin.roundtrip_any,
  (
    source: "any",
    enabled: true,
    count: 3,
    nested: (90deg, 25%, red),
    size: 12pt,
  ),
)
#roundtrip(plugin.roundtrip_any, (1, "two", 3.0, (nested: true)))
