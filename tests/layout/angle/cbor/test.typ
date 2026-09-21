#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-angle-cbor"), 90deg)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-angle-cbor"), -45deg)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-angle-cbor"), 0deg)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-angle-cbor"), 3.5rad)
