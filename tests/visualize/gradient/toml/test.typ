#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-gradient-toml"), gradient.linear(red, blue))
#roundtrip-format(typwire.toml, exports.at("roundtrip-gradient-toml"), gradient.linear(red, green, blue, angle: 45deg))
#roundtrip-format(typwire.toml, exports.at("roundtrip-gradient-toml"),
  gradient.radial(red, blue, center: (30%, 70%), radius: 40%),
)
#roundtrip-format(typwire.toml, exports.at("roundtrip-gradient-toml"), gradient.conic(red, blue, angle: 90deg))
