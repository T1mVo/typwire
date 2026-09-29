#import "../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-char-cbor, "x")
#roundtrip-format(typwire.cbor, plugin.roundtrip-char-cbor, "🦀")
