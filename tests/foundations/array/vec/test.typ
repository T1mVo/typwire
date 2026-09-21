#import "../../../common.typ": roundtrip, plugin

#roundtrip(plugin.roundtrip_vec, ())
#roundtrip(plugin.roundtrip_vec, (1, 2, 3))
