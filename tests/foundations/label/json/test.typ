#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-label-json, label("intro"))
#roundtrip-format(typwire.json, plugin.roundtrip-label-json, label("with-dashes_and.dots:colons"))

// The constructor accepts names the dedicated `<...>` syntax couldn't
// express, e.g. spaces and non-ASCII characters.
#roundtrip-format(typwire.json, plugin.roundtrip-label-json, label("weird chars! 名前"))
