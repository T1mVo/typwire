#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-none-json"), none)
#roundtrip-format(typwire.json, exports.at("roundtrip-none-json"), 12pt)
