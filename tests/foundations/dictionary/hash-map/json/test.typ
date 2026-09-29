#import "../../../../common.typ": roundtrip-format, plugin, typwire

// `HashMap` iteration order is unspecified; a single entry sidesteps that.
#roundtrip-format(typwire.json, plugin.roundtrip-hash-map-json, (:))
#roundtrip-format(typwire.json, plugin.roundtrip-hash-map-json, (only: 7))
