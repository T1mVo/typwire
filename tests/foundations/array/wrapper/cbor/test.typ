#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-array-wrapper-cbor, ())
#roundtrip-format(typwire.cbor, plugin.roundtrip-array-wrapper-cbor, (1, 2, 3))
