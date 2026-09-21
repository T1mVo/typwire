#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-gradient-json"), gradient.linear(red, blue))
#roundtrip-format(typwire.json, exports.at("roundtrip-gradient-json"), gradient.linear(red, green, blue, angle: 45deg))
#roundtrip-format(typwire.json, exports.at("roundtrip-gradient-json"),
  gradient.radial(red, blue, center: (30%, 70%), radius: 40%),
)
#roundtrip-format(typwire.json, exports.at("roundtrip-gradient-json"), gradient.conic(red, blue, angle: 90deg))
