#import "../../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-bool-wrapper-cbor, true)
#roundtrip-format(typwire.cbor, plugin.roundtrip-bool-wrapper-cbor, false)
