#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-u8-json"), 0)
#roundtrip-format(typwire.json, exports.at("roundtrip-u8-json"), 255)
