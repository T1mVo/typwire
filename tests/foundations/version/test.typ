#import "../../common.typ": roundtrip, plugin

#roundtrip(plugin.roundtrip_version, version(1, 2, 3))
#roundtrip(plugin.roundtrip_version, version(0, 15, 0))
#roundtrip(plugin.roundtrip_version, version(2, 0, 1, 4, 7))
