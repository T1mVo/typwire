#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-vec-json"), ())
#roundtrip-format(typwire.json, exports.at("roundtrip-vec-json"), (1, 2, 3))
