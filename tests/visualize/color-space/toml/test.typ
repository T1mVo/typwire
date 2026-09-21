#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-color-space-toml"), "luma")
#roundtrip-format(typwire.toml, exports.at("roundtrip-color-space-toml"), "oklab")
#roundtrip-format(typwire.toml, exports.at("roundtrip-color-space-toml"), "oklch")
#roundtrip-format(typwire.toml, exports.at("roundtrip-color-space-toml"), "linear-rgb")
#roundtrip-format(typwire.toml, exports.at("roundtrip-color-space-toml"), "rgb")
#roundtrip-format(typwire.toml, exports.at("roundtrip-color-space-toml"), "cmyk")
#roundtrip-format(typwire.toml, exports.at("roundtrip-color-space-toml"), "hsl")
#roundtrip-format(typwire.toml, exports.at("roundtrip-color-space-toml"), "hsv")
