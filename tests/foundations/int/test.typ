#import "../../common.typ": roundtrip, plugin

#roundtrip(plugin.roundtrip_int, 0)
#roundtrip(plugin.roundtrip_int, 42)
#roundtrip(plugin.roundtrip_int, -42)
