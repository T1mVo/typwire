#import "../../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-bool-wrapper-json, true)
#roundtrip-format(typwire.json, plugin.roundtrip-bool-wrapper-json, false)
