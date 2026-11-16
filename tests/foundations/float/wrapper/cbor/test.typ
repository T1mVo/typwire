#import "../../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-float-wrapper-cbor, 0.0)
#roundtrip-format(typwire.cbor, plugin.roundtrip-float-wrapper-cbor, 3.5)
#roundtrip-format(typwire.cbor, plugin.roundtrip-float-wrapper-cbor, -1.25)
