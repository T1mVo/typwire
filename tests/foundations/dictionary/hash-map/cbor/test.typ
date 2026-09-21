#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `HashMap` iteration order is unspecified; a single entry sidesteps that.
#roundtrip-format(typwire.cbor, exports.at("roundtrip-hash-map-cbor"), (:))
#roundtrip-format(typwire.cbor, exports.at("roundtrip-hash-map-cbor"), (only: 7))
