#import "../common.typ": check, plugin

#check(plugin.roundtrip_duration, duration(hours: 2))
#check(plugin.roundtrip_duration, duration(days: 1, minutes: 30))
#check(plugin.roundtrip_duration, duration(seconds: -90))
