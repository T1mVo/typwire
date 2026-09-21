#import "../../../common.typ": roundtrip, plugin

// `HashMap` iteration order is unspecified; a single entry sidesteps that.
#roundtrip(plugin.roundtrip_hash_map, (:))
#roundtrip(plugin.roundtrip_hash_map, (only: 7))
