#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.yaml, exports.at("roundtrip-label-yaml"), label("intro"))
#roundtrip-format(typwire.yaml, exports.at("roundtrip-label-yaml"), label("with-dashes_and.dots:colons"))

// The constructor accepts names the dedicated `<...>` syntax couldn't
// express, e.g. spaces and non-ASCII characters.
#roundtrip-format(typwire.yaml, exports.at("roundtrip-label-yaml"), label("weird chars! 名前"))
