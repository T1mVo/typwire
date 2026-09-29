#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-bool-wrapper-yaml, true)
#roundtrip-format(typwire.yaml, plugin.roundtrip-bool-wrapper-yaml, false)
