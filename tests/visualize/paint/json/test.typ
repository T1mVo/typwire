#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `ColorGradient` is untagged (`Color` or `Gradient`), so both native shapes
// must resolve correctly.
#roundtrip-format(typwire.json, exports.at("roundtrip-paint-json"), rgb("#1b1f8a"))
#roundtrip-format(typwire.json, exports.at("roundtrip-paint-json"), gradient.linear(red, blue))
#roundtrip-format(typwire.json, exports.at("roundtrip-paint-json"),
  gradient.radial(red, blue, center: (30%, 70%), radius: 40%),
)
