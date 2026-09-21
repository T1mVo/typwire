#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-center-cbor"), (x: 30%, y: 70%))
#roundtrip-format(typwire.cbor, exports.at("roundtrip-center-cbor"), (x: 0%, y: 100%))
