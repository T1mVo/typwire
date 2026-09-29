#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-color-cbor, luma(40%))
#roundtrip-format(typwire.cbor, plugin.roundtrip-color-cbor, rgb("#1b1f8a"))
#roundtrip-format(typwire.cbor, plugin.roundtrip-color-cbor, cmyk(10%, 20%, 30%, 40%))
#roundtrip-format(typwire.cbor, plugin.roundtrip-color-cbor, color.hsl(120deg, 50%, 60%))
#roundtrip-format(typwire.cbor, plugin.roundtrip-color-cbor, color.hsv(200deg, 30%, 70%))
#roundtrip-format(typwire.cbor, plugin.roundtrip-color-cbor, oklab(60%, 0.1, -0.1))
#roundtrip-format(typwire.cbor, plugin.roundtrip-color-cbor, oklch(60%, 0.1, 200deg))
#roundtrip-format(typwire.cbor, plugin.roundtrip-color-cbor, color.linear-rgb(20%, 40%, 60%))
