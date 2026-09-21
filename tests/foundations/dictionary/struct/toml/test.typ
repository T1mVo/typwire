#import "../../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// Backed by a `#[derive(Typwire)]` struct with fields `x`, `y`, `label`.
#roundtrip-format(typwire.toml, exports.at("roundtrip-struct-toml"), (x: 1.5, y: -2.0, label: "p"))
#roundtrip-format(typwire.toml, exports.at("roundtrip-struct-toml"), (x: 0.0, y: 0.0, label: ""))
