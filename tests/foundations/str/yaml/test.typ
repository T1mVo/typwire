#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-str-yaml"), "")
#roundtrip-format(typwire.yaml, exports.at("roundtrip-str-yaml"), "hello, typst")
