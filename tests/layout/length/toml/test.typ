#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-length-toml"), 12pt)
#roundtrip-format(typwire.toml, exports.at("roundtrip-length-toml"), 2.5cm)
#roundtrip-format(typwire.toml, exports.at("roundtrip-length-toml"), -3in)
#roundtrip-format(typwire.toml, exports.at("roundtrip-length-toml"), 0pt)
