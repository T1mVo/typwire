#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-angle-json"), 90deg)
#roundtrip-format(typwire.json, exports.at("roundtrip-angle-json"), -45deg)
#roundtrip-format(typwire.json, exports.at("roundtrip-angle-json"), 0deg)
#roundtrip-format(typwire.json, exports.at("roundtrip-angle-json"), 3.5rad)
