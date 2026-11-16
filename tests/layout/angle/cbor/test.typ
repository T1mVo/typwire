#import "../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-angle-cbor, 90deg)
#roundtrip-format(typwire.cbor, plugin.roundtrip-angle-cbor, -45deg)
#roundtrip-format(typwire.cbor, plugin.roundtrip-angle-cbor, 0deg)
#roundtrip-format(typwire.cbor, plugin.roundtrip-angle-cbor, 3.5rad)
