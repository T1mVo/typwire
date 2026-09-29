#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-float-wrapper-toml, 0.0)
#roundtrip-format(typwire.toml, plugin.roundtrip-float-wrapper-toml, 3.5)
#roundtrip-format(typwire.toml, plugin.roundtrip-float-wrapper-toml, -1.25)
