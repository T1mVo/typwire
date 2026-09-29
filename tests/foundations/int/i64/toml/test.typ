#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-i64-toml, 0)
#roundtrip-format(typwire.toml, plugin.roundtrip-i64-toml, 42)
#roundtrip-format(typwire.toml, plugin.roundtrip-i64-toml, -42)
