#import "../common.typ": roundtrip, plugin

// `OneOf3<i64, f64, String>` — variants tried in declaration order.
#roundtrip(plugin.roundtrip_one_of, 42)
#roundtrip(plugin.roundtrip_one_of, 3.5)
#roundtrip(plugin.roundtrip_one_of, "hello")
