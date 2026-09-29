#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-bool-wrapper-cbor, true)
#roundtrip-format(typwire.cbor, plugin.roundtrip-bool-wrapper-cbor, false)
