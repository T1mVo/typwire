#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-float-wrapper-cbor"), 0.0)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-float-wrapper-cbor"), 3.5)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-float-wrapper-cbor"), -1.25)
