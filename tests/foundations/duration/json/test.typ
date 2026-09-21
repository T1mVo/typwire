#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-duration-json"), duration(hours: 2))
#roundtrip-format(typwire.json, exports.at("roundtrip-duration-json"), duration(days: 1, minutes: 30))
#roundtrip-format(typwire.json, exports.at("roundtrip-duration-json"), duration(seconds: -90))
