#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-i64-json"), 0)
#roundtrip-format(typwire.json, exports.at("roundtrip-i64-json"), 42)
#roundtrip-format(typwire.json, exports.at("roundtrip-i64-json"), -42)
