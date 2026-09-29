#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-version-yaml, version(1, 2, 3))
#roundtrip-format(typwire.yaml, plugin.roundtrip-version-yaml, version(0, 15, 0))
#roundtrip-format(typwire.yaml, plugin.roundtrip-version-yaml, version(2, 0, 1, 4, 7))

// `version` accepts any number of components -- zero, one, or more than
// the traditional major/minor/patch/revision/build five -- so all of these
// must round-trip too, not just the 3- and 5-component cases above.
#roundtrip-format(typwire.yaml, plugin.roundtrip-version-yaml, version())
#roundtrip-format(typwire.yaml, plugin.roundtrip-version-yaml, version(1))
#roundtrip-format(typwire.yaml, plugin.roundtrip-version-yaml, version(1, 2, 3, 4, 5, 6, 7))

// `roundtrip-format` accepts `==`-equality as a pass, but `version`
// equality pads the shorter side with trailing zeros
// (`version(1, 2) == version(1, 2, 0, 0, 0)` is `true`), so it alone
// wouldn't actually prove the real component count survives the trip --
// only `repr` distinguishes them. Check that directly for a case with
// neither 0 nor 5 components, where a regression back to a fixed-width
// encoding would silently pass the `==` check above but fail this one.
#let original = version(1, 2)
#let decoded = typwire.yaml.call(plugin.roundtrip-version-yaml, original)
#assert.eq(repr(decoded), repr(original))
