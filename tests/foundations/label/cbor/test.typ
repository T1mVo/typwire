#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-label-cbor, label("intro"))
#roundtrip-format(typwire.cbor, plugin.roundtrip-label-cbor, label("with-dashes_and.dots:colons"))

// The constructor accepts names the dedicated `<...>` syntax couldn't
// express, e.g. spaces and non-ASCII characters.
#roundtrip-format(typwire.cbor, plugin.roundtrip-label-cbor, label("weird chars! 名前"))
