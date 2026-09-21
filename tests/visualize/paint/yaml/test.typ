#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `ColorGradient` is untagged (`Color` or `Gradient`), so both native shapes
// must resolve correctly.
#roundtrip-format(typwire.yaml, exports.at("roundtrip-paint-yaml"), rgb("#1b1f8a"))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-paint-yaml"), gradient.linear(red, blue))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-paint-yaml"),
  gradient.radial(red, blue, center: (30%, 70%), radius: 40%),
)
