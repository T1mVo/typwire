#import "../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-char-json, "x")
#roundtrip-format(typwire.json, plugin.roundtrip-char-json, "🦀")
