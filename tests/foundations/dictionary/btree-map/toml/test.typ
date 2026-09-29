#import "../../../../common.typ": roundtrip-format, plugin, typwire

// `BTreeMap` always re-serializes with keys in sorted order.
#roundtrip-format(typwire.toml, plugin.roundtrip-btree-map-toml, (:))
#roundtrip-format(typwire.toml, plugin.roundtrip-btree-map-toml, (a: 1, b: 2, c: 3))
