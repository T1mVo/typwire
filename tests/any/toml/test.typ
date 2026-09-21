#import "../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// Primitives.
#roundtrip-format(typwire.toml, exports.at("roundtrip-any-toml"), none)
#roundtrip-format(typwire.toml, exports.at("roundtrip-any-toml"), auto)
#roundtrip-format(typwire.toml, exports.at("roundtrip-any-toml"), true)
#roundtrip-format(typwire.toml, exports.at("roundtrip-any-toml"), 42)
#roundtrip-format(typwire.toml, exports.at("roundtrip-any-toml"), 3.5)
#roundtrip-format(typwire.toml, exports.at("roundtrip-any-toml"), "text")
#roundtrip-format(typwire.toml, exports.at("roundtrip-any-toml"), bytes((1, 2, 3)))
#roundtrip-format(typwire.toml, exports.at("roundtrip-any-toml"), path("test.typ"))
#roundtrip-format(typwire.toml, exports.at("roundtrip-any-toml"), symbol("★"))
#roundtrip-format(typwire.toml, exports.at("roundtrip-any-toml"), regex("a.*b"))
#roundtrip-format(typwire.toml, exports.at("roundtrip-any-toml"), decimal("3.14"))
#roundtrip-format(typwire.toml, exports.at("roundtrip-any-toml"), arguments(1, "two", a: 3))
#roundtrip-format(typwire.toml, exports.at("roundtrip-any-toml"), label("intro"))
#roundtrip-format(typwire.toml, exports.at("roundtrip-any-toml"), left + top)
#roundtrip-format(typwire.toml, exports.at("roundtrip-any-toml"), stroke(paint: red, thickness: 2pt))

// Nested containers mixing native Typst values.
#roundtrip-format(typwire.toml, exports.at("roundtrip-any-toml"),
  (
    source: "any",
    enabled: true,
    count: 3,
    nested: (90deg, 25%, red),
    size: 12pt,
  ),
)
#roundtrip-format(typwire.toml, exports.at("roundtrip-any-toml"), (1, "two", 3.0, (nested: true)))
