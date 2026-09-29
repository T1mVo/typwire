#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-duration-yaml, duration(hours: 2))
#roundtrip-format(typwire.yaml, plugin.roundtrip-duration-yaml, duration(days: 1, minutes: 30))
#roundtrip-format(typwire.yaml, plugin.roundtrip-duration-yaml, duration(seconds: -90))
