#import "../../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-i64-json, 0)
#roundtrip-format(typwire.json, plugin.roundtrip-i64-json, 42)
#roundtrip-format(typwire.json, plugin.roundtrip-i64-json, -42)
