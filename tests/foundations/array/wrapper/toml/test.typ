#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-array-wrapper-toml, ())
#roundtrip-format(typwire.toml, plugin.roundtrip-array-wrapper-toml, (1, 2, 3))
