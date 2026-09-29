#import "../../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-dictionary-wrapper-json, (:))
#roundtrip-format(typwire.json, plugin.roundtrip-dictionary-wrapper-json, (a: 1, b: 2, c: 3))
