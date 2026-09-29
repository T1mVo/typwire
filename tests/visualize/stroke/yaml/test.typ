#import "../../../common.typ": roundtrip-format, plugin, typwire

// All fields auto (the default stroke).
#roundtrip-format(typwire.yaml, plugin.roundtrip-stroke-yaml, stroke())

// A fully specified stroke: solid paint, explicit cap/join/dash/miter-limit.
#roundtrip-format(typwire.yaml, plugin.roundtrip-stroke-yaml,
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
#roundtrip-format(typwire.yaml, plugin.roundtrip-stroke-yaml, 2pt + gradient.linear(red, blue))

// `dash: none` (explicitly no dash, distinct from `auto`).
#roundtrip-format(typwire.yaml, plugin.roundtrip-stroke-yaml, stroke(dash: none))

// A custom dash array mixing lengths and the `"dot"` marker, plus a phase.
#roundtrip-format(typwire.yaml, plugin.roundtrip-stroke-yaml,
  stroke(dash: (array: (10pt, 5pt, "dot", 5pt), phase: 3pt)),
)

// The `2pt + red` shorthand.
#roundtrip-format(typwire.yaml, plugin.roundtrip-stroke-yaml, 2pt + red)
