#import "../../common.typ": roundtrip-format, plugin, typwire

#let rt(func, value) = roundtrip-format(typwire.toml, func, value)

// A native type (already a tagged dictionary at the wire level, so it
// satisfies TOML's top-level-table requirement for free).
#rt(plugin.roundtrip_format_toml_length, 12pt)

// `bytes`: TOML has no native byte-string type, so this exercises the
// tagged-array rewrite in `wire/rewrite-bytes.typ` -- which, as a side
// effect, also turns a bare top-level `bytes` value into the dictionary
// shape TOML requires at the root.
#rt(plugin.roundtrip_format_toml_bytes, bytes((1, 2, 3, 255)))
#rt(plugin.roundtrip_format_toml_bytes, bytes(()))

// `Any` holding a nested container, mixing an int and a float to exercise
// the int/float distinction across the TOML boundary.
#rt(
  plugin.roundtrip_format_toml_any,
  (source: "toml", count: 3, ratio: 0.5, data: bytes((1, 2))),
)

// A user struct.
#rt(plugin.roundtrip_format_toml_struct, (x: 1.5, label: "p"))

// A bare top-level scalar (e.g. `typwire.toml.encode(42)`) has no valid
// TOML representation and panics -- confirmed against Typst's own
// `toml.encode`, which rejects a bare scalar the same way. Not exercised
// here: Typst script has no `try`/`catch` to assert a panic occurs.
