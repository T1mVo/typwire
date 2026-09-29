#import "../../common.typ": roundtrip-format, plugin, typwire

// Functions renamed via `#[typwire::export(rename = "...")]`. Kebab-case
// names are reachable directly as `plugin.some-name` -- `dictionary(plugin)`
// is only needed below to check membership (`not in`), not to look up a
// function by name.
#let exports = dictionary(plugin)

// 1 argument.
#roundtrip-format(typwire.yaml, plugin.round-trip-yaml, 12pt)
#roundtrip-format(typwire.yaml, plugin.round-trip-yaml, -3in)

// 0 arguments.
#assert.eq(typwire.yaml.call(plugin.rename-nullary-yaml), 7)

// 2 arguments (exercises the argument-buffer split).
#assert.eq(typwire.yaml.call(plugin.add-lengths-yaml, 10pt, 2pt), 12pt)
#assert.eq(typwire.yaml.call(plugin.add-lengths-yaml, 100pt, -60pt), 40pt)

// The Rust identifiers themselves are not exported.
#for name in ("renamed_length_yaml", "renamed_add_yaml", "renamed_nullary_yaml") {
  assert(name not in exports, message: "unexpected `" + name + "` export")
}
