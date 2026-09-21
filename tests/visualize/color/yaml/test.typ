#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-color-yaml"), luma(40%))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-color-yaml"), rgb("#1b1f8a"))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-color-yaml"), cmyk(10%, 20%, 30%, 40%))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-color-yaml"), color.hsl(120deg, 50%, 60%))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-color-yaml"), color.hsv(200deg, 30%, 70%))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-color-yaml"), oklab(60%, 0.1, -0.1))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-color-yaml"), oklch(60%, 0.1, 200deg))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-color-yaml"), color.linear-rgb(20%, 40%, 60%))
