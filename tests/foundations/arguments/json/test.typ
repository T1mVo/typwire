#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-arguments-json, arguments())
#roundtrip-format(typwire.json, plugin.roundtrip-arguments-json, arguments(1, "two", 3.0))
#roundtrip-format(typwire.json, plugin.roundtrip-arguments-json, arguments(a: 1, b: "two"))
#roundtrip-format(typwire.json, plugin.roundtrip-arguments-json, arguments(1, "two", a: 3, b: 4))
#roundtrip-format(typwire.json, plugin.roundtrip-arguments-json,
  arguments(12pt, red, nested: (1, 2, 3), tag: none),
)
