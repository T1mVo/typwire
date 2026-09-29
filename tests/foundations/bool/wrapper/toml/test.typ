#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-bool-wrapper-toml, true)
#roundtrip-format(typwire.toml, plugin.roundtrip-bool-wrapper-toml, false)
