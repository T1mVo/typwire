#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-bytes-json"), bytes(()))
#roundtrip-format(typwire.json, exports.at("roundtrip-bytes-json"), bytes((1, 2, 3)))
