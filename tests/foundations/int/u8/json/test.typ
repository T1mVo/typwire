#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-u8-json, 0)
#roundtrip-format(typwire.json, plugin.roundtrip-u8-json, 255)
