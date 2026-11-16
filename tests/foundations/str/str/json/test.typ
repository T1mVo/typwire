#import "../../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-str-json, "")
#roundtrip-format(typwire.json, plugin.roundtrip-str-json, "hello, typst")
