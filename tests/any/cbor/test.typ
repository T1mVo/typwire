#import "../../common.typ": roundtrip-format, plugin, typwire

// Primitives.
#roundtrip-format(typwire.cbor, plugin.roundtrip-any-cbor, none)
#roundtrip-format(typwire.cbor, plugin.roundtrip-any-cbor, auto)
#roundtrip-format(typwire.cbor, plugin.roundtrip-any-cbor, true)
#roundtrip-format(typwire.cbor, plugin.roundtrip-any-cbor, 42)
#roundtrip-format(typwire.cbor, plugin.roundtrip-any-cbor, 3.5)
#roundtrip-format(typwire.cbor, plugin.roundtrip-any-cbor, "text")
#roundtrip-format(typwire.cbor, plugin.roundtrip-any-cbor, bytes((1, 2, 3)))
#roundtrip-format(typwire.cbor, plugin.roundtrip-any-cbor, path("test.typ"))
#roundtrip-format(typwire.cbor, plugin.roundtrip-any-cbor, symbol("★"))
#roundtrip-format(typwire.cbor, plugin.roundtrip-any-cbor, regex("a.*b"))
#roundtrip-format(typwire.cbor, plugin.roundtrip-any-cbor, decimal("3.14"))
#roundtrip-format(typwire.cbor, plugin.roundtrip-any-cbor, arguments(1, "two", a: 3))
#roundtrip-format(typwire.cbor, plugin.roundtrip-any-cbor, label("intro"))
#roundtrip-format(typwire.cbor, plugin.roundtrip-any-cbor, left + top)
#roundtrip-format(typwire.cbor, plugin.roundtrip-any-cbor, stroke(paint: red, thickness: 2pt))

// Nested containers mixing native Typst values.
#roundtrip-format(typwire.cbor, plugin.roundtrip-any-cbor,
  (
    source: "any",
    enabled: true,
    count: 3,
    nested: (90deg, 25%, red),
    size: 12pt,
  ),
)
#roundtrip-format(typwire.cbor, plugin.roundtrip-any-cbor, (1, "two", 3.0, (nested: true)))
