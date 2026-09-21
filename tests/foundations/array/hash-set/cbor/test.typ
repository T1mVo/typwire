#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `HashSet` iteration order is unspecified; a single element sidesteps that.
#roundtrip-format(typwire.cbor, exports.at("roundtrip-hash-set-cbor"), ())
#roundtrip-format(typwire.cbor, exports.at("roundtrip-hash-set-cbor"), (7,))
