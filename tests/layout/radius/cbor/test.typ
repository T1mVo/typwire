#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `Radius` decodes `left`/`top`/`right`/`bottom`/`rest` shorthands into the
// four canonical per-corner fields, but always re-encodes just those four
// (with the shorthands cleared to `none`). A value only survives a full
// round trip when it is already in that canonical shape.
#let corners(top-left: none, top-right: none, bottom-left: none, bottom-right: none) = (
  top-left: top-left,
  top-right: top-right,
  bottom-left: bottom-left,
  bottom-right: bottom-right,
  left: none,
  top: none,
  right: none,
  bottom: none,
  rest: none,
)

#roundtrip-format(typwire.cbor, exports.at("roundtrip-radius-cbor"), corners(top-left: 5pt, bottom-right: 3pt))
#roundtrip-format(typwire.cbor, exports.at("roundtrip-radius-cbor"), corners())

// The shorthands themselves are exercised directly against the resolved
// canonical shape, since they are not fixed points of the round trip.
#assert.eq(
  typwire.cbor.call(exports.at("roundtrip-radius-cbor"), (left: 5pt, top: 3pt)),
  corners(top-left: 5pt, top-right: 3pt, bottom-left: 5pt),
)
#assert.eq(
  typwire.cbor.call(exports.at("roundtrip-radius-cbor"), (rest: 2pt)),
  corners(top-left: 2pt, top-right: 2pt, bottom-left: 2pt, bottom-right: 2pt),
)
