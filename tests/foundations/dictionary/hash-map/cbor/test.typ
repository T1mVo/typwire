#import "../../../../common.typ": plugin, roundtrip-format, typwire

// `HashMap` iteration order is unspecified; a single entry sidesteps that.
#roundtrip-format(typwire.cbor, plugin.roundtrip-hash-map-cbor, (:))
#roundtrip-format(typwire.cbor, plugin.roundtrip-hash-map-cbor, (only: 7))
