#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-dictionary-wrapper-cbor"), (:))
#roundtrip-format(typwire.cbor, exports.at("roundtrip-dictionary-wrapper-cbor"), (a: 1, b: 2, c: 3))
