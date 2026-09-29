#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-color-json, luma(40%))
#roundtrip-format(typwire.json, plugin.roundtrip-color-json, rgb("#1b1f8a"))
#roundtrip-format(typwire.json, plugin.roundtrip-color-json, cmyk(10%, 20%, 30%, 40%))
#roundtrip-format(typwire.json, plugin.roundtrip-color-json, color.hsl(120deg, 50%, 60%))
#roundtrip-format(typwire.json, plugin.roundtrip-color-json, color.hsv(200deg, 30%, 70%))
#roundtrip-format(typwire.json, plugin.roundtrip-color-json, oklab(60%, 0.1, -0.1))
#roundtrip-format(typwire.json, plugin.roundtrip-color-json, oklch(60%, 0.1, 200deg))
#roundtrip-format(typwire.json, plugin.roundtrip-color-json, color.linear-rgb(20%, 40%, 60%))
