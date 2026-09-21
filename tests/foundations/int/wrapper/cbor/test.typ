#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-int-wrapper-cbor"), 0)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-int-wrapper-cbor"), 42)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-int-wrapper-cbor"), -42)
