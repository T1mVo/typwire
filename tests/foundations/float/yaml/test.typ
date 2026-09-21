#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-float-yaml"), 0.0)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-float-yaml"), 3.5)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-float-yaml"), -1.25)
