#import "../common.typ": check, plugin

// Primitives.
#check(plugin.roundtrip_any, none)
#check(plugin.roundtrip_any, true)
#check(plugin.roundtrip_any, 42)
#check(plugin.roundtrip_any, 3.5)
#check(plugin.roundtrip_any, "text")
#check(plugin.roundtrip_any, bytes((1, 2, 3)))

// Nested containers mixing native Typst values.
#check(
  plugin.roundtrip_any,
  (
    source: "any",
    enabled: true,
    count: 3,
    nested: (90deg, 25%, red),
    size: 12pt,
  ),
)
#check(plugin.roundtrip_any, (1, "two", 3.0, (nested: true)))
