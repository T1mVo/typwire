#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// Backed by a `#[derive(Typwire)]` struct with fields `x`, `y`, `label`.
#roundtrip-format(typwire.cbor, exports.at("roundtrip-struct-cbor"), (x: 1.5, y: -2.0, label: "p"))
#roundtrip-format(typwire.cbor, exports.at("roundtrip-struct-cbor"), (x: 0.0, y: 0.0, label: ""))
