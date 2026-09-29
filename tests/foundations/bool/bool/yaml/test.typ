#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-bool-yaml, true)
#roundtrip-format(typwire.yaml, plugin.roundtrip-bool-yaml, false)
