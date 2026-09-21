#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `BTreeMap` always re-serializes with keys in sorted order.
#roundtrip-format(typwire.yaml, exports.at("roundtrip-btree-map-yaml"), (:))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-btree-map-yaml"), (a: 1, b: 2, c: 3))
