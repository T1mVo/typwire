#import "../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-char-toml"), "x")
#roundtrip-format(typwire.toml, exports.at("roundtrip-char-toml"), "🦀")
