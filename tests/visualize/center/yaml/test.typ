#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-center-yaml"), (x: 30%, y: 70%))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-center-yaml"), (x: 0%, y: 100%))
