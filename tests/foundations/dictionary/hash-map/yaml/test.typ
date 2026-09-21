#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `HashMap` iteration order is unspecified; a single entry sidesteps that.
#roundtrip-format(typwire.yaml, exports.at("roundtrip-hash-map-yaml"), (:))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-hash-map-yaml"), (only: 7))
