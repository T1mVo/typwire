#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.toml, exports.at("roundtrip-label-toml"), label("intro"))
#roundtrip-format(typwire.toml, exports.at("roundtrip-label-toml"), label("with-dashes_and.dots:colons"))

// The constructor accepts names the dedicated `<...>` syntax couldn't
// express, e.g. spaces and non-ASCII characters.
#roundtrip-format(typwire.toml, exports.at("roundtrip-label-toml"), label("weird chars! 名前"))
