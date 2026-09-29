#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-str-wrapper-json, "")
#roundtrip-format(typwire.json, plugin.roundtrip-str-wrapper-json, "hello, typst")
