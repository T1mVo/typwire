#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-str-cbor, "")
#roundtrip-format(typwire.cbor, plugin.roundtrip-str-cbor, "hello, typst")
