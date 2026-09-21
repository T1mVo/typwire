#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-vec-cbor"), ())
#roundtrip-format(typwire.cbor, exports.at("roundtrip-vec-cbor"), (1, 2, 3))
