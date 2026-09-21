#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-u8-cbor"), 0)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-u8-cbor"), 255)
