#import "../../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-str-wrapper-json, "")
#roundtrip-format(
  typwire.json,
  plugin.roundtrip-str-wrapper-json,
  "hello, typst",
)
