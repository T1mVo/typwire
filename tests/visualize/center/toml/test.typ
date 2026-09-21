#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-center-toml"), (x: 30%, y: 70%))
#roundtrip-format(typwire.toml, exports.at("roundtrip-center-toml"), (x: 0%, y: 100%))
