#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-dictionary-wrapper-toml, (:))
#roundtrip-format(typwire.toml, plugin.roundtrip-dictionary-wrapper-toml, (a: 1, b: 2, c: 3))
