#import "../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(
  typwire.json,
  plugin.roundtrip-duration-json,
  duration(hours: 2),
)
#roundtrip-format(typwire.json, plugin.roundtrip-duration-json, duration(
  days: 1,
  minutes: 30,
))
#roundtrip-format(typwire.json, plugin.roundtrip-duration-json, duration(
  seconds: -90,
))
