#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-angle-yaml"), 90deg)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-angle-yaml"), -45deg)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-angle-yaml"), 0deg)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-angle-yaml"), 3.5rad)
