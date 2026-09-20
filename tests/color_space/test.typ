#import "../common.typ": roundtrip, plugin

#roundtrip(plugin.roundtrip_color_space, "luma")
#roundtrip(plugin.roundtrip_color_space, "oklab")
#roundtrip(plugin.roundtrip_color_space, "oklch")
#roundtrip(plugin.roundtrip_color_space, "linear-rgb")
#roundtrip(plugin.roundtrip_color_space, "rgb")
#roundtrip(plugin.roundtrip_color_space, "cmyk")
#roundtrip(plugin.roundtrip_color_space, "hsl")
#roundtrip(plugin.roundtrip_color_space, "hsv")
