#import "../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-char-json, "x")
#roundtrip-format(typwire.json, plugin.roundtrip-char-json, "🦀")
