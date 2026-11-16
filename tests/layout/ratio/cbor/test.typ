#import "../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-ratio-cbor, 25%)
#roundtrip-format(typwire.cbor, plugin.roundtrip-ratio-cbor, 140%)
#roundtrip-format(typwire.cbor, plugin.roundtrip-ratio-cbor, 0%)
