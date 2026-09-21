#import "../../common.typ": roundtrip-format, plugin, typwire

// Functions renamed via `#[typwire::export(rename = "...")]`. Non-identifier
// export names are reachable from Typst through `dictionary(plugin)`.
#let exports = dictionary(plugin)

// 1 argument.
#roundtrip-format(typwire.cbor, exports.at("round-trip-cbor"), 12pt)
#roundtrip-format(typwire.cbor, exports.at("round-trip-cbor"), -3in)

// 0 arguments.
#assert.eq(typwire.cbor.call(exports.at("rename-nullary-cbor")), 7)

// 2 arguments (exercises the argument-buffer split).
#assert.eq(typwire.cbor.call(exports.at("add-lengths-cbor"), 10pt, 2pt), 12pt)
#assert.eq(typwire.cbor.call(exports.at("add-lengths-cbor"), 100pt, -60pt), 40pt)

// The Rust identifiers themselves are not exported.
#for name in ("renamed_length_cbor", "renamed_add_cbor", "renamed_nullary_cbor") {
  assert(name not in exports, message: "unexpected `" + name + "` export")
}
