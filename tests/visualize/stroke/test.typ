#import "../../common.typ": roundtrip, plugin

// All fields auto (the default stroke).
#roundtrip(plugin.roundtrip_stroke, stroke())

// A fully specified stroke: solid paint, explicit cap/join/dash/miter-limit.
#roundtrip(
  plugin.roundtrip_stroke,
  stroke(
    paint: red,
    thickness: 2pt,
    cap: "round",
    join: "bevel",
    dash: "dashed",
    miter-limit: 5.0,
  ),
)

// Gradient paint.
#roundtrip(plugin.roundtrip_stroke, 2pt + gradient.linear(red, blue))

// `dash: none` (explicitly no dash, distinct from `auto`).
#roundtrip(plugin.roundtrip_stroke, stroke(dash: none))

// A custom dash array mixing lengths and the `"dot"` marker, plus a phase.
#roundtrip(
  plugin.roundtrip_stroke,
  stroke(dash: (array: (10pt, 5pt, "dot", 5pt), phase: 3pt)),
)

// The `2pt + red` shorthand.
#roundtrip(plugin.roundtrip_stroke, 2pt + red)
