#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-float-wrapper-toml"), 0.0)
#roundtrip-format(typwire.toml, exports.at("roundtrip-float-wrapper-toml"), 3.5)
#roundtrip-format(typwire.toml, exports.at("roundtrip-float-wrapper-toml"), -1.25)
