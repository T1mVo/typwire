#import "../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-none-cbor, none)
#roundtrip-format(typwire.cbor, plugin.roundtrip-none-cbor, 12pt)
