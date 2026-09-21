#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-str-wrapper-toml"), "")
#roundtrip-format(typwire.toml, exports.at("roundtrip-str-wrapper-toml"), "hello, typst")
