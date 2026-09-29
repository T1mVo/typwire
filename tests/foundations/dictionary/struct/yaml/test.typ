#import "../../../../common.typ": roundtrip-format, plugin, typwire

// Backed by a `#[derive(Typwire)]` struct with fields `x`, `y`, `label`.
#roundtrip-format(typwire.yaml, plugin.roundtrip-struct-yaml, (x: 1.5, y: -2.0, label: "p"))
#roundtrip-format(typwire.yaml, plugin.roundtrip-struct-yaml, (x: 0.0, y: 0.0, label: ""))
