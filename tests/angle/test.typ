#import "/tests/roundtrip.typ": check, plugin

#check(plugin.roundtrip_angle, 90deg)
#check(plugin.roundtrip_angle, -45deg)
#check(plugin.roundtrip_angle, 0deg)
#check(plugin.roundtrip_angle, 3.5rad)
