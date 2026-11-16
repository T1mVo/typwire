#import "../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-bytes-cbor, bytes(()))
#roundtrip-format(typwire.cbor, plugin.roundtrip-bytes-cbor, bytes((1, 2, 3)))
