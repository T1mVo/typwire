#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-length-cbor"), 12pt)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-length-cbor"), 2.5cm)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-length-cbor"), -3in)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-length-cbor"), 0pt)
