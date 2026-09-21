#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-float-cbor"), 0.0)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-float-cbor"), 3.5)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-float-cbor"), -1.25)
