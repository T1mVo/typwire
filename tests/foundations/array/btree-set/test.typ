#import "../../../common.typ": roundtrip, plugin

// `BTreeSet` always re-serializes in sorted order.
#roundtrip(plugin.roundtrip_btree_set, ())
#roundtrip(plugin.roundtrip_btree_set, (1, 2, 3))
