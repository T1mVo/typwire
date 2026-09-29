#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-bool-wrapper-json, true)
#roundtrip-format(typwire.json, plugin.roundtrip-bool-wrapper-json, false)
