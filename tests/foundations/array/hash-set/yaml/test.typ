#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `HashSet` iteration order is unspecified; a single element sidesteps that.
#roundtrip-format(typwire.yaml, exports.at("roundtrip-hash-set-yaml"), ())
#roundtrip-format(typwire.yaml, exports.at("roundtrip-hash-set-yaml"), (7,))
