#import "../../common.typ": roundtrip-format, plugin, typwire

#let rt(func, value) = roundtrip-format(typwire.json, func, value)

// A primitive.
#rt(plugin.roundtrip_format_json_i64, 42)

// A native type (a tagged dictionary at the wire level).
#rt(plugin.roundtrip_format_json_length, 12pt)

// `bytes`: JSON has no native byte-string type, so this exercises the
// tagged-array rewrite in `wire/rewrite-bytes.typ`.
#rt(plugin.roundtrip_format_json_bytes, bytes((1, 2, 3, 255)))
#rt(plugin.roundtrip_format_json_bytes, bytes(()))

// `Any` holding bytes and a nested container, mixing an int and a float to
// exercise the int/float distinction across the JSON boundary.
#rt(plugin.roundtrip_format_json_any, bytes((9, 8, 7)))
#rt(
  plugin.roundtrip_format_json_any,
  (source: "json", count: 3, ratio: 0.5, data: bytes((1, 2))),
)

// A user struct.
#rt(plugin.roundtrip_format_json_struct, (x: 1.5, label: "p"))
