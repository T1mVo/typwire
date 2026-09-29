#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-vec-json, ())
#roundtrip-format(typwire.json, plugin.roundtrip-vec-json, (1, 2, 3))
