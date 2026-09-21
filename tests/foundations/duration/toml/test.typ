#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-duration-toml"), duration(hours: 2))
#roundtrip-format(typwire.toml, exports.at("roundtrip-duration-toml"), duration(days: 1, minutes: 30))
#roundtrip-format(typwire.toml, exports.at("roundtrip-duration-toml"), duration(seconds: -90))
