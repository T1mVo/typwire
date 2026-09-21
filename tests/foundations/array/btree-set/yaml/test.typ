#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `BTreeSet` always re-serializes in sorted order.
#roundtrip-format(typwire.yaml, exports.at("roundtrip-btree-set-yaml"), ())
#roundtrip-format(typwire.yaml, exports.at("roundtrip-btree-set-yaml"), (1, 2, 3))
