#import "../common.typ": roundtrip, plugin

#roundtrip(plugin.roundtrip_vec, ())
#roundtrip(plugin.roundtrip_vec, (1, 2, 3))

#roundtrip(plugin.roundtrip_vec_deque, (1, 2, 3))

#roundtrip(plugin.roundtrip_linked_list, (1, 2, 3))

// `BTreeSet` always re-serializes in sorted order.
#roundtrip(plugin.roundtrip_btree_set, ())
#roundtrip(plugin.roundtrip_btree_set, (1, 2, 3))

// `HashSet` iteration order is unspecified; a single element sidesteps that.
#roundtrip(plugin.roundtrip_hash_set, ())
#roundtrip(plugin.roundtrip_hash_set, (7,))

#roundtrip(plugin.roundtrip_array, (1, 2, 3))

#roundtrip(plugin.roundtrip_box, 42)
#roundtrip(plugin.roundtrip_box, -1)

// `BTreeMap` always re-serializes with keys in sorted order.
#roundtrip(plugin.roundtrip_btree_map, (:))
#roundtrip(plugin.roundtrip_btree_map, (a: 1, b: 2, c: 3))

// `HashMap` iteration order is unspecified; a single entry sidesteps that.
#roundtrip(plugin.roundtrip_hash_map, (:))
#roundtrip(plugin.roundtrip_hash_map, (only: 7))

#roundtrip(plugin.roundtrip_tuple, (1, "two", 3.5))
