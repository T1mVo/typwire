#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-int-wrapper-cbor, 0)
#roundtrip-format(typwire.cbor, plugin.roundtrip-int-wrapper-cbor, 42)
#roundtrip-format(typwire.cbor, plugin.roundtrip-int-wrapper-cbor, -42)
