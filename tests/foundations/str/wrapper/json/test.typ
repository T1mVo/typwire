#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-str-wrapper-json"), "")
#roundtrip-format(typwire.json, exports.at("roundtrip-str-wrapper-json"), "hello, typst")
