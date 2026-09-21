#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `ColorGradient` is untagged (`Color` or `Gradient`), so both native shapes
// must resolve correctly.
#roundtrip-format(typwire.toml, exports.at("roundtrip-paint-toml"), rgb("#1b1f8a"))
#roundtrip-format(typwire.toml, exports.at("roundtrip-paint-toml"), gradient.linear(red, blue))
#roundtrip-format(typwire.toml, exports.at("roundtrip-paint-toml"),
  gradient.radial(red, blue, center: (30%, 70%), radius: 40%),
)
