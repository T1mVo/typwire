#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-ratio-json"), 25%)
#roundtrip-format(typwire.json, exports.at("roundtrip-ratio-json"), 140%)
#roundtrip-format(typwire.json, exports.at("roundtrip-ratio-json"), 0%)
