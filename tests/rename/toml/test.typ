#import "../../common.typ": roundtrip-format, plugin, typwire

// Functions renamed via `#[typwire::export(rename = "...")]`. Non-identifier
// export names are reachable from Typst through `dictionary(plugin)`.
#let exports = dictionary(plugin)

// 1 argument.
#roundtrip-format(typwire.toml, exports.at("round-trip-toml"), 12pt)
#roundtrip-format(typwire.toml, exports.at("round-trip-toml"), -3in)

// 0 arguments.
#assert.eq(typwire.toml.call(exports.at("rename-nullary-toml")), 7)

// 2 arguments (exercises the argument-buffer split).
#assert.eq(typwire.toml.call(exports.at("add-lengths-toml"), 10pt, 2pt), 12pt)
#assert.eq(typwire.toml.call(exports.at("add-lengths-toml"), 100pt, -60pt), 40pt)

// The Rust identifiers themselves are not exported.
#for name in ("renamed_length_toml", "renamed_add_toml", "renamed_nullary_toml") {
  assert(name not in exports, message: "unexpected `" + name + "` export")
}
