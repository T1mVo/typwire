#import "../../../../common.typ": roundtrip-format, plugin, typwire

// `HashMap` iteration order is unspecified; a single entry sidesteps that.
#roundtrip-format(typwire.toml, plugin.roundtrip-hash-map-toml, (:))
#roundtrip-format(typwire.toml, plugin.roundtrip-hash-map-toml, (only: 7))
