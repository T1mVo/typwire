#import "../../common.typ": roundtrip-format, plugin, typwire

// `OneOf3<i64, f64, String>` — variants tried in declaration order.
#roundtrip-format(typwire.yaml, plugin.roundtrip-one-of-yaml, 42)
#roundtrip-format(typwire.yaml, plugin.roundtrip-one-of-yaml, 3.5)
#roundtrip-format(typwire.yaml, plugin.roundtrip-one-of-yaml, "hello")
