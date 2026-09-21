#import "../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-char-cbor"), "x")
#roundtrip-format(typwire.cbor, exports.at("roundtrip-char-cbor"), "🦀")
