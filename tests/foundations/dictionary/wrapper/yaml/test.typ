#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-dictionary-wrapper-yaml"), (:))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-dictionary-wrapper-yaml"), (a: 1, b: 2, c: 3))
