#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `path` has no `==`-independent representation issue here: `Path`'s wire
// form is always the absolute, root-resolved form, which is what a
// roundtrip lands back on regardless of how the input was spelled.
#roundtrip-format(typwire.json, exports.at("roundtrip-path-json"), path("test.typ"))
#roundtrip-format(typwire.json, exports.at("roundtrip-path-json"), path("/assets/logo.png"))
#roundtrip-format(typwire.json, exports.at("roundtrip-path-json"), path("nested/dir/file.typ"))

// Regression test for `path.typ`'s hand-rolled un-escaping of `repr`'s
// quoted string literal (deliberately not `eval`, to avoid executing the
// path text as code) -- a literal quote in a path segment must round-trip
// through that escaping intact.
#roundtrip-format(typwire.json, exports.at("roundtrip-path-json"), path("weird \"quoted\" name.typ"))
