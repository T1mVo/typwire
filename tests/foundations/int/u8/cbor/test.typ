#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-u8-cbor, 0)
#roundtrip-format(typwire.cbor, plugin.roundtrip-u8-cbor, 255)
