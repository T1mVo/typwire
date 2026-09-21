#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-i64-toml"), 0)
#roundtrip-format(typwire.toml, exports.at("roundtrip-i64-toml"), 42)
#roundtrip-format(typwire.toml, exports.at("roundtrip-i64-toml"), -42)
