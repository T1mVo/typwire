#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-str-wrapper-yaml, "")
#roundtrip-format(typwire.yaml, plugin.roundtrip-str-wrapper-yaml, "hello, typst")
