#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-arguments-toml, arguments())
#roundtrip-format(typwire.toml, plugin.roundtrip-arguments-toml, arguments(1, "two", 3.0))
#roundtrip-format(typwire.toml, plugin.roundtrip-arguments-toml, arguments(a: 1, b: "two"))
#roundtrip-format(typwire.toml, plugin.roundtrip-arguments-toml, arguments(1, "two", a: 3, b: 4))
#roundtrip-format(typwire.toml, plugin.roundtrip-arguments-toml,
  arguments(12pt, red, nested: (1, 2, 3), tag: none),
)
