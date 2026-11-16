#import "../../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-i64-cbor, 0)
#roundtrip-format(typwire.cbor, plugin.roundtrip-i64-cbor, 42)
#roundtrip-format(typwire.cbor, plugin.roundtrip-i64-cbor, -42)
