#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-str-wrapper-cbor"), "")
#roundtrip-format(typwire.cbor, exports.at("roundtrip-str-wrapper-cbor"), "hello, typst")
