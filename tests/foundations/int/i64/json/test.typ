#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-i64-json, 0)
#roundtrip-format(typwire.json, plugin.roundtrip-i64-json, 42)
#roundtrip-format(typwire.json, plugin.roundtrip-i64-json, -42)
