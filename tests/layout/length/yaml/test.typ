#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-length-yaml"), 12pt)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-length-yaml"), 2.5cm)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-length-yaml"), -3in)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-length-yaml"), 0pt)
