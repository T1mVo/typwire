#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-bool-wrapper-json"), true)
#roundtrip-format(typwire.json, exports.at("roundtrip-bool-wrapper-json"), false)
