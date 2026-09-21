#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-float-wrapper-json"), 0.0)
#roundtrip-format(typwire.json, exports.at("roundtrip-float-wrapper-json"), 3.5)
#roundtrip-format(typwire.json, exports.at("roundtrip-float-wrapper-json"), -1.25)
