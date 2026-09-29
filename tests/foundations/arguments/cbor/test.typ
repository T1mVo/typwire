#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-arguments-cbor, arguments())
#roundtrip-format(typwire.cbor, plugin.roundtrip-arguments-cbor, arguments(1, "two", 3.0))
#roundtrip-format(typwire.cbor, plugin.roundtrip-arguments-cbor, arguments(a: 1, b: "two"))
#roundtrip-format(typwire.cbor, plugin.roundtrip-arguments-cbor, arguments(1, "two", a: 3, b: 4))
#roundtrip-format(typwire.cbor, plugin.roundtrip-arguments-cbor,
  arguments(12pt, red, nested: (1, 2, 3), tag: none),
)
