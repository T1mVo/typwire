#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-bool-cbor, true)
#roundtrip-format(typwire.cbor, plugin.roundtrip-bool-cbor, false)
