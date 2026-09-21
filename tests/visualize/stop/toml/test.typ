#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-stop-toml"), (color: rgb("#1b1f8a"), offset: 25%))
#roundtrip-format(typwire.toml, exports.at("roundtrip-stop-toml"), (color: luma(40%), offset: 0%))
