#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-float-yaml, 0.0)
#roundtrip-format(typwire.yaml, plugin.roundtrip-float-yaml, 3.5)
#roundtrip-format(typwire.yaml, plugin.roundtrip-float-yaml, -1.25)
