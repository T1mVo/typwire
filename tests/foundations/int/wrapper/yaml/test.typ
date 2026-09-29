#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-int-wrapper-yaml, 0)
#roundtrip-format(typwire.yaml, plugin.roundtrip-int-wrapper-yaml, 42)
#roundtrip-format(typwire.yaml, plugin.roundtrip-int-wrapper-yaml, -42)
