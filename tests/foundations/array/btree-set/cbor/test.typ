#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `BTreeSet` always re-serializes in sorted order.
#roundtrip-format(typwire.cbor, exports.at("roundtrip-btree-set-cbor"), ())
#roundtrip-format(typwire.cbor, exports.at("roundtrip-btree-set-cbor"), (1, 2, 3))
