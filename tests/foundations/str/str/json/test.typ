#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-str-json, "")
#roundtrip-format(typwire.json, plugin.roundtrip-str-json, "hello, typst")
