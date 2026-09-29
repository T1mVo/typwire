#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-float-toml, 0.0)
#roundtrip-format(typwire.toml, plugin.roundtrip-float-toml, 3.5)
#roundtrip-format(typwire.toml, plugin.roundtrip-float-toml, -1.25)
