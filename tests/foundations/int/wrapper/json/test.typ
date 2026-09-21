#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-int-wrapper-json"), 0)
#roundtrip-format(typwire.json, exports.at("roundtrip-int-wrapper-json"), 42)
#roundtrip-format(typwire.json, exports.at("roundtrip-int-wrapper-json"), -42)
