#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-dictionary-wrapper-json"), (:))
#roundtrip-format(typwire.json, exports.at("roundtrip-dictionary-wrapper-json"), (a: 1, b: 2, c: 3))
