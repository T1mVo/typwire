#import "../common.typ": roundtrip, plugin

// `LengthRadius` is untagged (`Length` or `Radius`), tried in that order.
#roundtrip(plugin.roundtrip_length_radius, 12pt)
#roundtrip(plugin.roundtrip_length_radius, -3in)

// A `Radius` value must already be in canonical per-corner shape to survive
// a full round trip; see `tests/radius/test.typ`.
#roundtrip(
  plugin.roundtrip_length_radius,
  (
    top-left: 5pt,
    top-right: none,
    bottom-left: none,
    bottom-right: 3pt,
    left: none,
    top: none,
    right: none,
    bottom: none,
    rest: none,
  ),
)
