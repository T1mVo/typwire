#import "../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-char-yaml, "x")
#roundtrip-format(typwire.yaml, plugin.roundtrip-char-yaml, "🦀")
