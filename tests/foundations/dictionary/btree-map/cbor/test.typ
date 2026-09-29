#import "../../../../common.typ": roundtrip-format, plugin, typwire

// `BTreeMap` always re-serializes with keys in sorted order.
#roundtrip-format(typwire.cbor, plugin.roundtrip-btree-map-cbor, (:))
#roundtrip-format(typwire.cbor, plugin.roundtrip-btree-map-cbor, (a: 1, b: 2, c: 3))
