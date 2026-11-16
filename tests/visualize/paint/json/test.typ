#import "../../../common.typ": plugin, roundtrip-format, typwire

// `ColorGradient` is untagged (`Color` or `Gradient`), so both native shapes
// must resolve correctly.
#roundtrip-format(typwire.json, plugin.roundtrip-paint-json, rgb("#1b1f8a"))
#roundtrip-format(typwire.json, plugin.roundtrip-paint-json, gradient.linear(
  red,
  blue,
))
#roundtrip-format(typwire.json, plugin.roundtrip-paint-json, gradient.radial(
  red,
  blue,
  center: (30%, 70%),
  radius: 40%,
))
