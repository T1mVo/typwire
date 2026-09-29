#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-color-toml, luma(40%))
#roundtrip-format(typwire.toml, plugin.roundtrip-color-toml, rgb("#1b1f8a"))
#roundtrip-format(typwire.toml, plugin.roundtrip-color-toml, cmyk(10%, 20%, 30%, 40%))
#roundtrip-format(typwire.toml, plugin.roundtrip-color-toml, color.hsl(120deg, 50%, 60%))
#roundtrip-format(typwire.toml, plugin.roundtrip-color-toml, color.hsv(200deg, 30%, 70%))
#roundtrip-format(typwire.toml, plugin.roundtrip-color-toml, oklab(60%, 0.1, -0.1))
#roundtrip-format(typwire.toml, plugin.roundtrip-color-toml, oklch(60%, 0.1, 200deg))
#roundtrip-format(typwire.toml, plugin.roundtrip-color-toml, color.linear-rgb(20%, 40%, 60%))
