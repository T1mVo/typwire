#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-color-space-yaml"), "luma")
#roundtrip-format(typwire.yaml, exports.at("roundtrip-color-space-yaml"), "oklab")
#roundtrip-format(typwire.yaml, exports.at("roundtrip-color-space-yaml"), "oklch")
#roundtrip-format(typwire.yaml, exports.at("roundtrip-color-space-yaml"), "linear-rgb")
#roundtrip-format(typwire.yaml, exports.at("roundtrip-color-space-yaml"), "rgb")
#roundtrip-format(typwire.yaml, exports.at("roundtrip-color-space-yaml"), "cmyk")
#roundtrip-format(typwire.yaml, exports.at("roundtrip-color-space-yaml"), "hsl")
#roundtrip-format(typwire.yaml, exports.at("roundtrip-color-space-yaml"), "hsv")
