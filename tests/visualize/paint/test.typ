#import "../../common.typ": roundtrip, plugin

// `ColorGradient` is untagged (`Color` or `Gradient`), so both native shapes
// must resolve correctly.
#roundtrip(plugin.roundtrip_paint, rgb("#1b1f8a"))
#roundtrip(plugin.roundtrip_paint, gradient.linear(red, blue))
#roundtrip(
  plugin.roundtrip_paint,
  gradient.radial(red, blue, center: (30%, 70%), radius: 40%),
)
