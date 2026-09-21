#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-bool-cbor"), true)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-bool-cbor"), false)
