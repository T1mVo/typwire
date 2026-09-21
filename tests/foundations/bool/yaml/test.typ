#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-bool-yaml"), true)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-bool-yaml"), false)
