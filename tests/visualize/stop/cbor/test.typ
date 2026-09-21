#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-stop-cbor"), (color: rgb("#1b1f8a"), offset: 25%))
#roundtrip-format(typwire.cbor, exports.at("roundtrip-stop-cbor"), (color: luma(40%), offset: 0%))
