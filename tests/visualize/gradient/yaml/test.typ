#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-gradient-yaml"), gradient.linear(red, blue))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-gradient-yaml"), gradient.linear(red, green, blue, angle: 45deg))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-gradient-yaml"),
  gradient.radial(red, blue, center: (30%, 70%), radius: 40%),
)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-gradient-yaml"), gradient.conic(red, blue, angle: 90deg))
