#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-bool-toml"), true)
#roundtrip-format(typwire.toml, exports.at("roundtrip-bool-toml"), false)
