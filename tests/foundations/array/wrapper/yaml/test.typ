#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-array-wrapper-yaml, ())
#roundtrip-format(typwire.yaml, plugin.roundtrip-array-wrapper-yaml, (1, 2, 3))
