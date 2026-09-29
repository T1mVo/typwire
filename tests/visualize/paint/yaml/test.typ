#import "../../../common.typ": roundtrip-format, plugin, typwire

// `ColorGradient` is untagged (`Color` or `Gradient`), so both native shapes
// must resolve correctly.
#roundtrip-format(typwire.yaml, plugin.roundtrip-paint-yaml, rgb("#1b1f8a"))
#roundtrip-format(typwire.yaml, plugin.roundtrip-paint-yaml, gradient.linear(red, blue))
#roundtrip-format(typwire.yaml, plugin.roundtrip-paint-yaml,
  gradient.radial(red, blue, center: (30%, 70%), radius: 40%),
)
