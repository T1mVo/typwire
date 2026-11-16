#import "../../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-vec-cbor, ())
#roundtrip-format(typwire.cbor, plugin.roundtrip-vec-cbor, (1, 2, 3))
