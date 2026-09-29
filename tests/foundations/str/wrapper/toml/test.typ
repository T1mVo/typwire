#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-str-wrapper-toml, "")
#roundtrip-format(typwire.toml, plugin.roundtrip-str-wrapper-toml, "hello, typst")
