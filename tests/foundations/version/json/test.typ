#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-version-json"), version(1, 2, 3))
#roundtrip-format(typwire.json, exports.at("roundtrip-version-json"), version(0, 15, 0))
#roundtrip-format(typwire.json, exports.at("roundtrip-version-json"), version(2, 0, 1, 4, 7))
