#import "../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `OneOf3<i64, f64, String>` — variants tried in declaration order.
#roundtrip-format(typwire.toml, exports.at("roundtrip-one-of-toml"), 42)
#roundtrip-format(typwire.toml, exports.at("roundtrip-one-of-toml"), 3.5)
#roundtrip-format(typwire.toml, exports.at("roundtrip-one-of-toml"), "hello")
