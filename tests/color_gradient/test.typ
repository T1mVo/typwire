#import "../common.typ": roundtrip, plugin

// `ColorGradient` is untagged (`Color` or `Gradient`), so both native shapes
// must resolve correctly.
#roundtrip(plugin.roundtrip_color_gradient, rgb("#1b1f8a"))
#roundtrip(plugin.roundtrip_color_gradient, gradient.linear(red, blue))
#roundtrip(
  plugin.roundtrip_color_gradient,
  gradient.radial(red, blue, center: (30%, 70%), radius: 40%),
)
