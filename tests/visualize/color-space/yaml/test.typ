#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-color-space-yaml, "luma")
#roundtrip-format(typwire.yaml, plugin.roundtrip-color-space-yaml, "oklab")
#roundtrip-format(typwire.yaml, plugin.roundtrip-color-space-yaml, "oklch")
#roundtrip-format(typwire.yaml, plugin.roundtrip-color-space-yaml, "linear-rgb")
#roundtrip-format(typwire.yaml, plugin.roundtrip-color-space-yaml, "rgb")
#roundtrip-format(typwire.yaml, plugin.roundtrip-color-space-yaml, "cmyk")
#roundtrip-format(typwire.yaml, plugin.roundtrip-color-space-yaml, "hsl")
#roundtrip-format(typwire.yaml, plugin.roundtrip-color-space-yaml, "hsv")
