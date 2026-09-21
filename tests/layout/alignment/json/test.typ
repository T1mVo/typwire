#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// Every horizontal alignment, standalone.
#roundtrip-format(typwire.json, exports.at("roundtrip-alignment-json"), start)
#roundtrip-format(typwire.json, exports.at("roundtrip-alignment-json"), left)
#roundtrip-format(typwire.json, exports.at("roundtrip-alignment-json"), center)
#roundtrip-format(typwire.json, exports.at("roundtrip-alignment-json"), right)
#roundtrip-format(typwire.json, exports.at("roundtrip-alignment-json"), end)

// Every vertical alignment, standalone.
#roundtrip-format(typwire.json, exports.at("roundtrip-alignment-json"), top)
#roundtrip-format(typwire.json, exports.at("roundtrip-alignment-json"), horizon)
#roundtrip-format(typwire.json, exports.at("roundtrip-alignment-json"), bottom)

// 2D combinations.
#roundtrip-format(typwire.json, exports.at("roundtrip-alignment-json"), left + top)
#roundtrip-format(typwire.json, exports.at("roundtrip-alignment-json"), right + bottom)
#roundtrip-format(typwire.json, exports.at("roundtrip-alignment-json"), center + horizon)
