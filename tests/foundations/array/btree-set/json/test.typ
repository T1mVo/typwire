#import "../../../../common.typ": roundtrip-format, plugin, typwire

// `BTreeSet` always re-serializes in sorted order.
#roundtrip-format(typwire.json, plugin.roundtrip-btree-set-json, ())
#roundtrip-format(typwire.json, plugin.roundtrip-btree-set-json, (1, 2, 3))
