#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

// `path` has no `==`-independent representation issue here: `Path`'s wire
// form is always the absolute, root-resolved form, which is what a
// roundtrip lands back on regardless of how the input was spelled.
#roundtrip-format(typwire.toml, exports.at("roundtrip-path-toml"), path("test.typ"))
#roundtrip-format(typwire.toml, exports.at("roundtrip-path-toml"), path("/assets/logo.png"))
#roundtrip-format(typwire.toml, exports.at("roundtrip-path-toml"), path("nested/dir/file.typ"))

// Regression test for `path.typ`'s hand-rolled un-escaping of `repr`'s
// quoted string literal (deliberately not `eval`, to avoid executing the
// path text as code) -- a literal quote in a path segment must round-trip
// through that escaping intact.
#roundtrip-format(typwire.toml, exports.at("roundtrip-path-toml"), path("weird \"quoted\" name.typ"))
