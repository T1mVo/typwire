#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-stop-json"), (color: rgb("#1b1f8a"), offset: 25%))
#roundtrip-format(typwire.json, exports.at("roundtrip-stop-json"), (color: luma(40%), offset: 0%))
