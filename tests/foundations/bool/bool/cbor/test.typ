#import "../../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-bool-cbor, true)
#roundtrip-format(typwire.cbor, plugin.roundtrip-bool-cbor, false)
