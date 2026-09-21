#import "../../../common.typ": roundtrip, plugin

// `HashSet` iteration order is unspecified; a single element sidesteps that.
#roundtrip(plugin.roundtrip_hash_set, ())
#roundtrip(plugin.roundtrip_hash_set, (7,))
