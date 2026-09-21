#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-bytes-cbor"), bytes(()))
#roundtrip-format(typwire.cbor, exports.at("roundtrip-bytes-cbor"), bytes((1, 2, 3)))
