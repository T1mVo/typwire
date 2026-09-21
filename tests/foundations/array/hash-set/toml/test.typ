#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `HashSet` iteration order is unspecified; a single element sidesteps that.
#roundtrip-format(typwire.toml, exports.at("roundtrip-hash-set-toml"), ())
#roundtrip-format(typwire.toml, exports.at("roundtrip-hash-set-toml"), (7,))
