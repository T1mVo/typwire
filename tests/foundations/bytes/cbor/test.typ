#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-bytes-cbor, bytes(()))
#roundtrip-format(typwire.cbor, plugin.roundtrip-bytes-cbor, bytes((1, 2, 3)))
