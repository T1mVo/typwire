#import "/tests/roundtrip.typ": check, plugin

#check(plugin.roundtrip_option, none)
#check(plugin.roundtrip_option, 12pt)
