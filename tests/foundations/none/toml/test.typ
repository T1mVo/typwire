#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-none-toml"), none)
#roundtrip-format(typwire.toml, exports.at("roundtrip-none-toml"), 12pt)
