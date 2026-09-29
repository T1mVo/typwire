#import "../../common.typ": roundtrip-format, plugin, typwire

// `OneOf3<i64, f64, String>` — variants tried in declaration order.
#roundtrip-format(typwire.json, plugin.roundtrip-one-of-json, 42)
#roundtrip-format(typwire.json, plugin.roundtrip-one-of-json, 3.5)
#roundtrip-format(typwire.json, plugin.roundtrip-one-of-json, "hello")
