#import "../../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-u8-cbor, 0)
#roundtrip-format(typwire.cbor, plugin.roundtrip-u8-cbor, 255)
