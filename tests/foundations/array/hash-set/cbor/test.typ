#import "../../../../common.typ": roundtrip-format, plugin, typwire

// `HashSet` iteration order is unspecified; a single element sidesteps that.
#roundtrip-format(typwire.cbor, plugin.roundtrip-hash-set-cbor, ())
#roundtrip-format(typwire.cbor, plugin.roundtrip-hash-set-cbor, (7,))
