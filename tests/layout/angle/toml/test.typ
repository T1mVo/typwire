#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-angle-toml"), 90deg)
#roundtrip-format(typwire.toml, exports.at("roundtrip-angle-toml"), -45deg)
#roundtrip-format(typwire.toml, exports.at("roundtrip-angle-toml"), 0deg)
#roundtrip-format(typwire.toml, exports.at("roundtrip-angle-toml"), 3.5rad)
