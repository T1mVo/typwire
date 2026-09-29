#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-ratio-cbor, 25%)
#roundtrip-format(typwire.cbor, plugin.roundtrip-ratio-cbor, 140%)
#roundtrip-format(typwire.cbor, plugin.roundtrip-ratio-cbor, 0%)
