#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-fixed-size-array-toml, (1, 2, 3))
