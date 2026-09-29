#import "../../../../common.typ": roundtrip-format, plugin, typwire

// `BTreeSet` always re-serializes in sorted order.
#roundtrip-format(typwire.cbor, plugin.roundtrip-btree-set-cbor, ())
#roundtrip-format(typwire.cbor, plugin.roundtrip-btree-set-cbor, (1, 2, 3))
