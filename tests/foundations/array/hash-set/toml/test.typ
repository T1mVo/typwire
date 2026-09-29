#import "../../../../common.typ": roundtrip-format, plugin, typwire

// `HashSet` iteration order is unspecified; a single element sidesteps that.
#roundtrip-format(typwire.toml, plugin.roundtrip-hash-set-toml, ())
#roundtrip-format(typwire.toml, plugin.roundtrip-hash-set-toml, (7,))
