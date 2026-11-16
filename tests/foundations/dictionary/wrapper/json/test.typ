#import "../../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-dictionary-wrapper-json, (:))
#roundtrip-format(typwire.json, plugin.roundtrip-dictionary-wrapper-json, (
  a: 1,
  b: 2,
  c: 3,
))
