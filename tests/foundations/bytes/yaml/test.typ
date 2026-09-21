#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-bytes-yaml"), bytes(()))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-bytes-yaml"), bytes((1, 2, 3)))
