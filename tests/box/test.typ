// `Box<T>` is wire-transparent -- it has no shape of its own distinct from
// `T`, so it doesn't belong under `tests/foundations/` alongside the types
// that actually have a Typst-side identity.
#import "../common.typ": roundtrip, plugin

#roundtrip(plugin.roundtrip_box, 42)
#roundtrip(plugin.roundtrip_box, -1)
