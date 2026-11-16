#import "../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-char-cbor, "x")
#roundtrip-format(typwire.cbor, plugin.roundtrip-char-cbor, "🦀")
