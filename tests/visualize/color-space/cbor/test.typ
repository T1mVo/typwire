#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-color-space-cbor"), "luma")
#roundtrip-format(typwire.cbor, exports.at("roundtrip-color-space-cbor"), "oklab")
#roundtrip-format(typwire.cbor, exports.at("roundtrip-color-space-cbor"), "oklch")
#roundtrip-format(typwire.cbor, exports.at("roundtrip-color-space-cbor"), "linear-rgb")
#roundtrip-format(typwire.cbor, exports.at("roundtrip-color-space-cbor"), "rgb")
#roundtrip-format(typwire.cbor, exports.at("roundtrip-color-space-cbor"), "cmyk")
#roundtrip-format(typwire.cbor, exports.at("roundtrip-color-space-cbor"), "hsl")
#roundtrip-format(typwire.cbor, exports.at("roundtrip-color-space-cbor"), "hsv")
