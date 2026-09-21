#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-center-json"), (x: 30%, y: 70%))
#roundtrip-format(typwire.json, exports.at("roundtrip-center-json"), (x: 0%, y: 100%))
