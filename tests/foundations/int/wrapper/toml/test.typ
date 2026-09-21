#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-int-wrapper-toml"), 0)
#roundtrip-format(typwire.toml, exports.at("roundtrip-int-wrapper-toml"), 42)
#roundtrip-format(typwire.toml, exports.at("roundtrip-int-wrapper-toml"), -42)
