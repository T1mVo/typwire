#import "../../common.typ": roundtrip, plugin

#roundtrip(plugin.roundtrip_gradient, gradient.linear(red, blue))
#roundtrip(plugin.roundtrip_gradient, gradient.linear(red, green, blue, angle: 45deg))
#roundtrip(
  plugin.roundtrip_gradient,
  gradient.radial(red, blue, center: (30%, 70%), radius: 40%),
)
#roundtrip(plugin.roundtrip_gradient, gradient.conic(red, blue, angle: 90deg))
