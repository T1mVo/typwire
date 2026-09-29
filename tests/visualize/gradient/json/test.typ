#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-gradient-json, gradient.linear(red, blue))
#roundtrip-format(typwire.json, plugin.roundtrip-gradient-json, gradient.linear(red, green, blue, angle: 45deg))
#roundtrip-format(typwire.json, plugin.roundtrip-gradient-json,
  gradient.radial(red, blue, center: (30%, 70%), radius: 40%),
)
#roundtrip-format(typwire.json, plugin.roundtrip-gradient-json, gradient.conic(red, blue, angle: 90deg))
