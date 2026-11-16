#import "../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-auto-cbor, auto)
