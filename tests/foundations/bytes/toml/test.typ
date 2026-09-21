#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-bytes-toml"), bytes(()))
#roundtrip-format(typwire.toml, exports.at("roundtrip-bytes-toml"), bytes((1, 2, 3)))
