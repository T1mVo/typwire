#import "../../common.typ": roundtrip, plugin

#roundtrip(plugin.roundtrip_duration, duration(hours: 2))
#roundtrip(plugin.roundtrip_duration, duration(days: 1, minutes: 30))
#roundtrip(plugin.roundtrip_duration, duration(seconds: -90))
