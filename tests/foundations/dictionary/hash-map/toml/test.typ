#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `HashMap` iteration order is unspecified; a single entry sidesteps that.
#roundtrip-format(typwire.toml, exports.at("roundtrip-hash-map-toml"), (:))
#roundtrip-format(typwire.toml, exports.at("roundtrip-hash-map-toml"), (only: 7))
