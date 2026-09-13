#import "/tests/roundtrip.typ": check, plugin

#check(plugin.roundtrip_length, 12pt)
#check(plugin.roundtrip_length, 2.5cm)
#check(plugin.roundtrip_length, -3in)
#check(plugin.roundtrip_length, 0pt)
