#import "../../common.typ": roundtrip, plugin

#roundtrip(plugin.roundtrip_bytes, bytes(()))
#roundtrip(plugin.roundtrip_bytes, bytes((1, 2, 3)))
