#import "../../../../common.typ": roundtrip-format, plugin, typwire

// `HashSet` iteration order is unspecified; a single element sidesteps that.
#roundtrip-format(typwire.yaml, plugin.roundtrip-hash-set-yaml, ())
#roundtrip-format(typwire.yaml, plugin.roundtrip-hash-set-yaml, (7,))
