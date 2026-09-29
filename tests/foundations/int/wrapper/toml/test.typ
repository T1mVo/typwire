#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-int-wrapper-toml, 0)
#roundtrip-format(typwire.toml, plugin.roundtrip-int-wrapper-toml, 42)
#roundtrip-format(typwire.toml, plugin.roundtrip-int-wrapper-toml, -42)
