#import "../../../../common.typ": roundtrip-format, plugin, typwire

// `BTreeSet` always re-serializes in sorted order.
#roundtrip-format(typwire.toml, plugin.roundtrip-btree-set-toml, ())
#roundtrip-format(typwire.toml, plugin.roundtrip-btree-set-toml, (1, 2, 3))
