#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-str-wrapper-yaml"), "")
#roundtrip-format(typwire.yaml, exports.at("roundtrip-str-wrapper-yaml"), "hello, typst")
