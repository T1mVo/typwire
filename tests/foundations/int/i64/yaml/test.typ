#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-i64-yaml, 0)
#roundtrip-format(typwire.yaml, plugin.roundtrip-i64-yaml, 42)
#roundtrip-format(typwire.yaml, plugin.roundtrip-i64-yaml, -42)
