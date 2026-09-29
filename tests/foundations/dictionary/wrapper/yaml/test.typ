#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-dictionary-wrapper-yaml, (:))
#roundtrip-format(typwire.yaml, plugin.roundtrip-dictionary-wrapper-yaml, (a: 1, b: 2, c: 3))
