#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-array-wrapper-toml"), ())
#roundtrip-format(typwire.toml, exports.at("roundtrip-array-wrapper-toml"), (1, 2, 3))
