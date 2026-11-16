#import "../../../../common.typ": plugin, roundtrip-format, typwire

// `HashMap` iteration order is unspecified; a single entry sidesteps that.
#roundtrip-format(typwire.json, plugin.roundtrip-hash-map-json, (:))
#roundtrip-format(typwire.json, plugin.roundtrip-hash-map-json, (only: 7))
