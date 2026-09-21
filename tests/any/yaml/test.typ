#import "../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// Primitives.
#roundtrip-format(typwire.yaml, exports.at("roundtrip-any-yaml"), none)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-any-yaml"), auto)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-any-yaml"), true)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-any-yaml"), 42)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-any-yaml"), 3.5)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-any-yaml"), "text")
#roundtrip-format(typwire.yaml, exports.at("roundtrip-any-yaml"), bytes((1, 2, 3)))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-any-yaml"), path("test.typ"))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-any-yaml"), symbol("★"))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-any-yaml"), regex("a.*b"))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-any-yaml"), decimal("3.14"))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-any-yaml"), arguments(1, "two", a: 3))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-any-yaml"), label("intro"))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-any-yaml"), left + top)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-any-yaml"), stroke(paint: red, thickness: 2pt))

// Nested containers mixing native Typst values.
#roundtrip-format(typwire.yaml, exports.at("roundtrip-any-yaml"),
  (
    source: "any",
    enabled: true,
    count: 3,
    nested: (90deg, 25%, red),
    size: 12pt,
  ),
)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-any-yaml"), (1, "two", 3.0, (nested: true)))
