#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-vec-toml"), ())
#roundtrip-format(typwire.toml, exports.at("roundtrip-vec-toml"), (1, 2, 3))
