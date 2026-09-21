// `Box<T>` is wire-transparent -- it has no shape of its own distinct from
// `T`, so it doesn't belong under `tests/foundations/` alongside the types
// that actually have a Typst-side identity.
#import "../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-box-cbor"), 42)
#roundtrip-format(typwire.cbor, exports.at("roundtrip-box-cbor"), -1)
