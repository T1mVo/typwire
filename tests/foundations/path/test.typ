#import "../../common.typ": roundtrip, plugin

// `path` has no `==`-independent representation issue here: `Path`'s wire
// form is always the absolute, root-resolved form, which is what a
// roundtrip lands back on regardless of how the input was spelled.
#roundtrip(plugin.roundtrip_path, path("test.typ"))
#roundtrip(plugin.roundtrip_path, path("/assets/logo.png"))
#roundtrip(plugin.roundtrip_path, path("nested/dir/file.typ"))

// Regression test for `path.typ`'s hand-rolled un-escaping of `repr`'s
// quoted string literal (deliberately not `eval`, to avoid executing the
// path text as code) -- a literal quote in a path segment must round-trip
// through that escaping intact.
#roundtrip(plugin.roundtrip_path, path("weird \"quoted\" name.typ"))
