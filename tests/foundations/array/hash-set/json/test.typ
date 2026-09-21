#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `HashSet` iteration order is unspecified; a single element sidesteps that.
#roundtrip-format(typwire.json, exports.at("roundtrip-hash-set-json"), ())
#roundtrip-format(typwire.json, exports.at("roundtrip-hash-set-json"), (7,))
