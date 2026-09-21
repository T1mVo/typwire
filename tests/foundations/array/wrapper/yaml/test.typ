#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-array-wrapper-yaml"), ())
#roundtrip-format(typwire.yaml, exports.at("roundtrip-array-wrapper-yaml"), (1, 2, 3))
