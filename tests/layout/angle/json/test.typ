#import "../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-angle-json, 90deg)
#roundtrip-format(typwire.json, plugin.roundtrip-angle-json, -45deg)
#roundtrip-format(typwire.json, plugin.roundtrip-angle-json, 0deg)
#roundtrip-format(typwire.json, plugin.roundtrip-angle-json, 3.5rad)
