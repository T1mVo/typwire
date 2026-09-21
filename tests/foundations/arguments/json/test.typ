#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-arguments-json"), arguments())
#roundtrip-format(typwire.json, exports.at("roundtrip-arguments-json"), arguments(1, "two", 3.0))
#roundtrip-format(typwire.json, exports.at("roundtrip-arguments-json"), arguments(a: 1, b: "two"))
#roundtrip-format(typwire.json, exports.at("roundtrip-arguments-json"), arguments(1, "two", a: 3, b: 4))
#roundtrip-format(typwire.json, exports.at("roundtrip-arguments-json"),
  arguments(12pt, red, nested: (1, 2, 3), tag: none),
)
