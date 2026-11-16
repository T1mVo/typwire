#import "../../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-u8-json, 0)
#roundtrip-format(typwire.json, plugin.roundtrip-u8-json, 255)
