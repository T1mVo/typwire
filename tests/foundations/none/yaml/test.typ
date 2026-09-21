#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-none-yaml"), none)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-none-yaml"), 12pt)
