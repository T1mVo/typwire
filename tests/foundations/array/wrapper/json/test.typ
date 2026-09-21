#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-array-wrapper-json"), ())
#roundtrip-format(typwire.json, exports.at("roundtrip-array-wrapper-json"), (1, 2, 3))
