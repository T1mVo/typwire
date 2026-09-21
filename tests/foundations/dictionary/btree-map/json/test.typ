#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `BTreeMap` always re-serializes with keys in sorted order.
#roundtrip-format(typwire.json, exports.at("roundtrip-btree-map-json"), (:))
#roundtrip-format(typwire.json, exports.at("roundtrip-btree-map-json"), (a: 1, b: 2, c: 3))
