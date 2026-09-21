#import "../../common.typ": roundtrip, plugin

#roundtrip(plugin.roundtrip_type, int)
#roundtrip(plugin.roundtrip_type, str)
#roundtrip(plugin.roundtrip_type, bool)
#roundtrip(plugin.roundtrip_type, length)
#roundtrip(plugin.roundtrip_type, dictionary)
#roundtrip(plugin.roundtrip_type, type(none))
#roundtrip(plugin.roundtrip_type, type(auto))
#roundtrip(plugin.roundtrip_type, type(path("test.typ")))
#roundtrip(plugin.roundtrip_type, type(symbol("★")))
#roundtrip(plugin.roundtrip_type, type(regex("a")))
#roundtrip(plugin.roundtrip_type, type(decimal("1")))
#roundtrip(plugin.roundtrip_type, type(arguments()))
