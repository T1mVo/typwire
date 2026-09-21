#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `BTreeSet` always re-serializes in sorted order.
#roundtrip-format(typwire.toml, exports.at("roundtrip-btree-set-toml"), ())
#roundtrip-format(typwire.toml, exports.at("roundtrip-btree-set-toml"), (1, 2, 3))
