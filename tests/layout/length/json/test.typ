#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-length-json"), 12pt)
#roundtrip-format(typwire.json, exports.at("roundtrip-length-json"), 2.5cm)
#roundtrip-format(typwire.json, exports.at("roundtrip-length-json"), -3in)
#roundtrip-format(typwire.json, exports.at("roundtrip-length-json"), 0pt)
