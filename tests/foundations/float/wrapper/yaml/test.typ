#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-float-wrapper-yaml, 0.0)
#roundtrip-format(typwire.yaml, plugin.roundtrip-float-wrapper-yaml, 3.5)
#roundtrip-format(typwire.yaml, plugin.roundtrip-float-wrapper-yaml, -1.25)
