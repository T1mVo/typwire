#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-dictionary-wrapper-toml"), (:))
#roundtrip-format(typwire.toml, exports.at("roundtrip-dictionary-wrapper-toml"), (a: 1, b: 2, c: 3))
