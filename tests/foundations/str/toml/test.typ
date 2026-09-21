#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-str-toml"), "")
#roundtrip-format(typwire.toml, exports.at("roundtrip-str-toml"), "hello, typst")
