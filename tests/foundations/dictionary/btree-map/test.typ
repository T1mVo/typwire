#import "../../../common.typ": roundtrip, plugin

// `BTreeMap` always re-serializes with keys in sorted order.
#roundtrip(plugin.roundtrip_btree_map, (:))
#roundtrip(plugin.roundtrip_btree_map, (a: 1, b: 2, c: 3))
