#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-float-json"), 0.0)
#roundtrip-format(typwire.json, exports.at("roundtrip-float-json"), 3.5)
#roundtrip-format(typwire.json, exports.at("roundtrip-float-json"), -1.25)
