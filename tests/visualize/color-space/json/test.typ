#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-color-space-json, "luma")
#roundtrip-format(typwire.json, plugin.roundtrip-color-space-json, "oklab")
#roundtrip-format(typwire.json, plugin.roundtrip-color-space-json, "oklch")
#roundtrip-format(typwire.json, plugin.roundtrip-color-space-json, "linear-rgb")
#roundtrip-format(typwire.json, plugin.roundtrip-color-space-json, "rgb")
#roundtrip-format(typwire.json, plugin.roundtrip-color-space-json, "cmyk")
#roundtrip-format(typwire.json, plugin.roundtrip-color-space-json, "hsl")
#roundtrip-format(typwire.json, plugin.roundtrip-color-space-json, "hsv")
