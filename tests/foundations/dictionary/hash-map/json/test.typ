#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `HashMap` iteration order is unspecified; a single entry sidesteps that.
#roundtrip-format(typwire.json, exports.at("roundtrip-hash-map-json"), (:))
#roundtrip-format(typwire.json, exports.at("roundtrip-hash-map-json"), (only: 7))
