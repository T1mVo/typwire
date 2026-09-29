#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-vec-toml, ())
#roundtrip-format(typwire.toml, plugin.roundtrip-vec-toml, (1, 2, 3))
