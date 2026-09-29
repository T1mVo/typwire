#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-vec-cbor, ())
#roundtrip-format(typwire.cbor, plugin.roundtrip-vec-cbor, (1, 2, 3))
