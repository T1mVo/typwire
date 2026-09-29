#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-str-wrapper-cbor, "")
#roundtrip-format(typwire.cbor, plugin.roundtrip-str-wrapper-cbor, "hello, typst")
