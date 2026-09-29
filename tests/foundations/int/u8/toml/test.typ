#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-u8-toml, 0)
#roundtrip-format(typwire.toml, plugin.roundtrip-u8-toml, 255)
