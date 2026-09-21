#import "../../common.typ": roundtrip, plugin

#roundtrip(plugin.roundtrip_label, label("intro"))
#roundtrip(plugin.roundtrip_label, label("with-dashes_and.dots:colons"))

// The constructor accepts names the dedicated `<...>` syntax couldn't
// express, e.g. spaces and non-ASCII characters.
#roundtrip(plugin.roundtrip_label, label("weird chars! 名前"))
