#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-u8-yaml, 0)
#roundtrip-format(typwire.yaml, plugin.roundtrip-u8-yaml, 255)
