#import "../../common.typ": plugin, roundtrip-format, typwire

// `()` is a zero-length CBOR array, not `none` -- the latter is Option's
// wire shape.
#roundtrip-format(typwire.cbor, plugin.roundtrip-unit-cbor, ())
