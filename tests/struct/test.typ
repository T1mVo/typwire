#import "../common.typ": check, plugin

// Backed by a `#[derive(Typwire)]` struct with fields `x`, `y`, `label`.
#check(plugin.roundtrip_struct, (x: 1.5, y: -2.0, label: "p"))
#check(plugin.roundtrip_struct, (x: 0.0, y: 0.0, label: ""))
