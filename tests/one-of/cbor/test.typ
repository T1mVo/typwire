#import "../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `OneOf3<i64, f64, String>` — variants tried in declaration order.
#roundtrip-format(typwire.cbor, exports.at("roundtrip-one-of-cbor"), 42)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-one-of-cbor"), 3.5)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-one-of-cbor"), "hello")
