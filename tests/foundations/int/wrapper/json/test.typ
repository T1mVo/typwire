#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-int-wrapper-json, 0)
#roundtrip-format(typwire.json, plugin.roundtrip-int-wrapper-json, 42)
#roundtrip-format(typwire.json, plugin.roundtrip-int-wrapper-json, -42)
