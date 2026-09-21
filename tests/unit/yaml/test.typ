#import "../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `()` is a zero-length CBOR array, not `none` -- the latter is Option's
// wire shape.
#roundtrip-format(typwire.yaml, exports.at("roundtrip-unit-yaml"), ())
