#import "../../common.typ": roundtrip-format, plugin, typwire

#let rt(func, value) = roundtrip-format(typwire.yaml, func, value)

// A primitive.
#rt(plugin.roundtrip_format_yaml_i64, 42)

// A native type (a tagged dictionary at the wire level).
#rt(plugin.roundtrip_format_yaml_length, 12pt)

// `bytes`: YAML has no native byte-string type, so this exercises the
// tagged-array rewrite in `wire/rewrite-bytes.typ`.
#rt(plugin.roundtrip_format_yaml_bytes, bytes((1, 2, 3, 255)))
#rt(plugin.roundtrip_format_yaml_bytes, bytes(()))

// `Any` holding bytes and a nested container, mixing an int and a float to
// exercise the int/float distinction across the YAML boundary.
#rt(plugin.roundtrip_format_yaml_any, bytes((9, 8, 7)))
#rt(
  plugin.roundtrip_format_yaml_any,
  (source: "yaml", count: 3, ratio: 0.5, data: bytes((1, 2))),
)

// A user struct.
#rt(plugin.roundtrip_format_yaml_struct, (x: 1.5, label: "p"))
