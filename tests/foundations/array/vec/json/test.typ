#import "../../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-vec-json, ())
#roundtrip-format(typwire.json, plugin.roundtrip-vec-json, (1, 2, 3))
