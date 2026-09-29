#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-float-cbor, 0.0)
#roundtrip-format(typwire.cbor, plugin.roundtrip-float-cbor, 3.5)
#roundtrip-format(typwire.cbor, plugin.roundtrip-float-cbor, -1.25)
