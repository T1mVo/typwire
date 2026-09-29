#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-str-toml, "")
#roundtrip-format(typwire.toml, plugin.roundtrip-str-toml, "hello, typst")
