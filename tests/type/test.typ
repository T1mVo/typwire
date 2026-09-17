#import "../common.typ": check, plugin

#check(plugin.roundtrip_type, int)
#check(plugin.roundtrip_type, str)
#check(plugin.roundtrip_type, bool)
#check(plugin.roundtrip_type, length)
#check(plugin.roundtrip_type, dictionary)
