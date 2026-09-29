#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-str-yaml, "")
#roundtrip-format(typwire.yaml, plugin.roundtrip-str-yaml, "hello, typst")
