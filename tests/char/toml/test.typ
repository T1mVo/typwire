#import "../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-char-toml, "x")
#roundtrip-format(typwire.toml, plugin.roundtrip-char-toml, "🦀")
