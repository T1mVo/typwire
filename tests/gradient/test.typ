#import "../common.typ": check, plugin

#check(plugin.roundtrip_gradient, gradient.linear(red, blue))
#check(plugin.roundtrip_gradient, gradient.linear(red, green, blue, angle: 45deg))
#check(
  plugin.roundtrip_gradient,
  gradient.radial(red, blue, center: (30%, 70%), radius: 40%),
)
#check(plugin.roundtrip_gradient, gradient.conic(red, blue, angle: 90deg))
