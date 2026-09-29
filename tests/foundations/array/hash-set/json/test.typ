#import "../../../../common.typ": roundtrip-format, plugin, typwire

// `HashSet` iteration order is unspecified; a single element sidesteps that.
#roundtrip-format(typwire.json, plugin.roundtrip-hash-set-json, ())
#roundtrip-format(typwire.json, plugin.roundtrip-hash-set-json, (7,))
