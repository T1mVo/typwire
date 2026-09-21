#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-i64-cbor"), 0)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-i64-cbor"), 42)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-i64-cbor"), -42)
