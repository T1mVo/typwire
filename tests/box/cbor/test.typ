// `Box<T>` is wire-transparent -- it has no shape of its own distinct from
// `T`, so it doesn't belong under `tests/foundations/` alongside the types
// that actually have a Typst-side identity.
#import "../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-box-cbor, 42)
#roundtrip-format(typwire.cbor, plugin.roundtrip-box-cbor, -1)
