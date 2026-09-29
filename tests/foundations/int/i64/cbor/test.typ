#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-i64-cbor, 0)
#roundtrip-format(typwire.cbor, plugin.roundtrip-i64-cbor, 42)
#roundtrip-format(typwire.cbor, plugin.roundtrip-i64-cbor, -42)
