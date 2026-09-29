#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-vec-yaml, ())
#roundtrip-format(typwire.yaml, plugin.roundtrip-vec-yaml, (1, 2, 3))
