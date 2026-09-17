#import "../common.typ": check, plugin

#check(plugin.roundtrip_datetime, datetime(year: 2026, month: 8, day: 25))
#check(plugin.roundtrip_datetime, datetime(hour: 13, minute: 5, second: 0))
#check(
  plugin.roundtrip_datetime,
  datetime(year: 1999, month: 12, day: 31, hour: 23, minute: 59, second: 58),
)
