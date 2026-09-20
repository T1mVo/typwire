#import "../common.typ": roundtrip, plugin

// `()` is a zero-length CBOR array, not `none` -- the latter is Option's
// wire shape.
#roundtrip(plugin.roundtrip_unit, ())
