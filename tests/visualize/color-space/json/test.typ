#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-color-space-json"), "luma")
#roundtrip-format(typwire.json, exports.at("roundtrip-color-space-json"), "oklab")
#roundtrip-format(typwire.json, exports.at("roundtrip-color-space-json"), "oklch")
#roundtrip-format(typwire.json, exports.at("roundtrip-color-space-json"), "linear-rgb")
#roundtrip-format(typwire.json, exports.at("roundtrip-color-space-json"), "rgb")
#roundtrip-format(typwire.json, exports.at("roundtrip-color-space-json"), "cmyk")
#roundtrip-format(typwire.json, exports.at("roundtrip-color-space-json"), "hsl")
#roundtrip-format(typwire.json, exports.at("roundtrip-color-space-json"), "hsv")
