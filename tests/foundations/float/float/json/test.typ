#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-float-json, 0.0)
#roundtrip-format(typwire.json, plugin.roundtrip-float-json, 3.5)
#roundtrip-format(typwire.json, plugin.roundtrip-float-json, -1.25)
