#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `BTreeMap` always re-serializes with keys in sorted order.
#roundtrip-format(typwire.toml, exports.at("roundtrip-btree-map-toml"), (:))
#roundtrip-format(typwire.toml, exports.at("roundtrip-btree-map-toml"), (a: 1, b: 2, c: 3))
