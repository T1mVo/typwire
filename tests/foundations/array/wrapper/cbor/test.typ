#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-array-wrapper-cbor"), ())
#roundtrip-format(typwire.cbor, exports.at("roundtrip-array-wrapper-cbor"), (1, 2, 3))
