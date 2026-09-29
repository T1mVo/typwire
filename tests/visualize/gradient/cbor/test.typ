#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-gradient-cbor, gradient.linear(red, blue))
#roundtrip-format(typwire.cbor, plugin.roundtrip-gradient-cbor, gradient.linear(red, green, blue, angle: 45deg))
#roundtrip-format(typwire.cbor, plugin.roundtrip-gradient-cbor,
  gradient.radial(red, blue, center: (30%, 70%), radius: 40%),
)
#roundtrip-format(typwire.cbor, plugin.roundtrip-gradient-cbor, gradient.conic(red, blue, angle: 90deg))
