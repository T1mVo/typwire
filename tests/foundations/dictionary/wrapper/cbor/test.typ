#import "../../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-dictionary-wrapper-cbor, (:))
#roundtrip-format(typwire.cbor, plugin.roundtrip-dictionary-wrapper-cbor, (
  a: 1,
  b: 2,
  c: 3,
))
