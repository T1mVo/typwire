#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-none-cbor"), none)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-none-cbor"), 12pt)
