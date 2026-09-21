#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `BTreeSet` always re-serializes in sorted order.
#roundtrip-format(typwire.json, exports.at("roundtrip-btree-set-json"), ())
#roundtrip-format(typwire.json, exports.at("roundtrip-btree-set-json"), (1, 2, 3))
