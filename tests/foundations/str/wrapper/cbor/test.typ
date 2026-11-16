#import "../../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-str-wrapper-cbor, "")
#roundtrip-format(
  typwire.cbor,
  plugin.roundtrip-str-wrapper-cbor,
  "hello, typst",
)
