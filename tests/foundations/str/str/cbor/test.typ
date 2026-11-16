#import "../../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-str-cbor, "")
#roundtrip-format(typwire.cbor, plugin.roundtrip-str-cbor, "hello, typst")
