#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-vec-yaml"), ())
#roundtrip-format(typwire.yaml, exports.at("roundtrip-vec-yaml"), (1, 2, 3))
