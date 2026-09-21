#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-ratio-toml"), 25%)
#roundtrip-format(typwire.toml, exports.at("roundtrip-ratio-toml"), 140%)
#roundtrip-format(typwire.toml, exports.at("roundtrip-ratio-toml"), 0%)
