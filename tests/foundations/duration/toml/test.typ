#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-duration-toml, duration(hours: 2))
#roundtrip-format(typwire.toml, plugin.roundtrip-duration-toml, duration(days: 1, minutes: 30))
#roundtrip-format(typwire.toml, plugin.roundtrip-duration-toml, duration(seconds: -90))
