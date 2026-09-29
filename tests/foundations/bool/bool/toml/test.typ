#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-bool-toml, true)
#roundtrip-format(typwire.toml, plugin.roundtrip-bool-toml, false)
