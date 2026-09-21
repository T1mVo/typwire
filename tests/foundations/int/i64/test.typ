#import "../../../common.typ": roundtrip, plugin

#roundtrip(plugin.roundtrip_i64, 0)
#roundtrip(plugin.roundtrip_i64, 42)
#roundtrip(plugin.roundtrip_i64, -42)
