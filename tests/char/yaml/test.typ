#import "../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-char-yaml"), "x")
#roundtrip-format(typwire.yaml, exports.at("roundtrip-char-yaml"), "🦀")
