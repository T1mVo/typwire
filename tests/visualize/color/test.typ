#import "../../common.typ": roundtrip, plugin

#roundtrip(plugin.roundtrip_color, luma(40%))
#roundtrip(plugin.roundtrip_color, rgb("#1b1f8a"))
#roundtrip(plugin.roundtrip_color, cmyk(10%, 20%, 30%, 40%))
#roundtrip(plugin.roundtrip_color, color.hsl(120deg, 50%, 60%))
#roundtrip(plugin.roundtrip_color, color.hsv(200deg, 30%, 70%))
#roundtrip(plugin.roundtrip_color, oklab(60%, 0.1, -0.1))
#roundtrip(plugin.roundtrip_color, oklch(60%, 0.1, 200deg))
#roundtrip(plugin.roundtrip_color, color.linear-rgb(20%, 40%, 60%))
