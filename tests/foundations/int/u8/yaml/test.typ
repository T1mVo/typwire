#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-u8-yaml"), 0)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-u8-yaml"), 255)
