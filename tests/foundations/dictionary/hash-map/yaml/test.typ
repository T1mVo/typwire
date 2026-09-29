#import "../../../../common.typ": roundtrip-format, plugin, typwire

// `HashMap` iteration order is unspecified; a single entry sidesteps that.
#roundtrip-format(typwire.yaml, plugin.roundtrip-hash-map-yaml, (:))
#roundtrip-format(typwire.yaml, plugin.roundtrip-hash-map-yaml, (only: 7))
