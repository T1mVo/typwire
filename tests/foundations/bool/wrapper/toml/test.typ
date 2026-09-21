#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-bool-wrapper-toml"), true)
#roundtrip-format(typwire.toml, exports.at("roundtrip-bool-wrapper-toml"), false)
