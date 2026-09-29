#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-bool-json, true)
#roundtrip-format(typwire.json, plugin.roundtrip-bool-json, false)
