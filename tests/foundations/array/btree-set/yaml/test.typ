#import "../../../../common.typ": roundtrip-format, plugin, typwire

// `BTreeSet` always re-serializes in sorted order.
#roundtrip-format(typwire.yaml, plugin.roundtrip-btree-set-yaml, ())
#roundtrip-format(typwire.yaml, plugin.roundtrip-btree-set-yaml, (1, 2, 3))
