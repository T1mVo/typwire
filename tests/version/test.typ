#import "../common.typ": check, plugin

#check(plugin.roundtrip_version, version(1, 2, 3))
#check(plugin.roundtrip_version, version(0, 15, 0))
#check(plugin.roundtrip_version, version(2, 0, 1, 4, 7))
