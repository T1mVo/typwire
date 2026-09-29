#import "../../../../common.typ": roundtrip-format, plugin, typwire

// `BTreeMap` always re-serializes with keys in sorted order.
#roundtrip-format(typwire.json, plugin.roundtrip-btree-map-json, (:))
#roundtrip-format(typwire.json, plugin.roundtrip-btree-map-json, (a: 1, b: 2, c: 3))
