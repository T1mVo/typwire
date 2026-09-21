#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-ratio-yaml"), 25%)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-ratio-yaml"), 140%)
#roundtrip-format(typwire.yaml, exports.at("roundtrip-ratio-yaml"), 0%)
