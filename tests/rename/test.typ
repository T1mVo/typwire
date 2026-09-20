#import "../common.typ": roundtrip, plugin, typwire

// Functions renamed via `#[typwire::export(rename = "...")]`. Non-identifier
// export names are reachable from Typst through `dictionary(plugin)`.
#let exports = dictionary(plugin)

// 1 argument.
#roundtrip(exports.at("round-trip"), 12pt)
#roundtrip(exports.at("round-trip"), -3in)

// 0 arguments.
#assert.eq(typwire.call(exports.at("rename-nullary")), 7)

// 2 arguments (exercises the argument-buffer split).
#assert.eq(typwire.call(exports.at("add-lengths"), 10pt, 2pt), 12pt)
#assert.eq(typwire.call(exports.at("add-lengths"), 100pt, -60pt), 40pt)

// The Rust identifiers themselves are not exported.
#for name in ("renamed_length", "renamed_add", "renamed_nullary") {
  assert(name not in exports, message: "unexpected `" + name + "` export")
}
