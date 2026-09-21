#import "../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-char-json"), "x")
#roundtrip-format(typwire.json, exports.at("roundtrip-char-json"), "🦀")
