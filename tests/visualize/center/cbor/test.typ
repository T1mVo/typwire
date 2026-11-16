#import "../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-center-cbor, (x: 30%, y: 70%))
#roundtrip-format(typwire.cbor, plugin.roundtrip-center-cbor, (x: 0%, y: 100%))
