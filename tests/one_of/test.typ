#import "/tests/roundtrip.typ": check, plugin

// `OneOf3<i64, f64, String>` — variants tried in declaration order.
#check(plugin.roundtrip_one_of, 42)
#check(plugin.roundtrip_one_of, 3.5)
#check(plugin.roundtrip_one_of, "hello")
