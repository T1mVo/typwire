#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-label-json"), label("intro"))
#roundtrip-format(typwire.json, exports.at("roundtrip-label-json"), label("with-dashes_and.dots:colons"))

// The constructor accepts names the dedicated `<...>` syntax couldn't
// express, e.g. spaces and non-ASCII characters.
#roundtrip-format(typwire.json, exports.at("roundtrip-label-json"), label("weird chars! 名前"))
