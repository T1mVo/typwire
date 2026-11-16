#import "../../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-array-wrapper-json, ())
#roundtrip-format(typwire.json, plugin.roundtrip-array-wrapper-json, (1, 2, 3))
