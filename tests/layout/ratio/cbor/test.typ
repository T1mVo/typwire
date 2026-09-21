#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-ratio-cbor"), 25%)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-ratio-cbor"), 140%)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-ratio-cbor"), 0%)
