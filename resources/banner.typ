#let light = sys.inputs.at("theme", default: "light") == "light"

#let W = 1200pt
#let H = 265pt
#let m = 8pt // transparent margin around the card

// ── Palette ─────────────────────────────────────────────────────────────
#let accent = if light { rgb("#e5502a") } else { rgb("#ffa45c") }
#let ink = if light { rgb("#3a0d06") } else { white }
#let sub = if light { rgb("#a3361a") } else { rgb("#ffd3b3") }
#let tag = if light { rgb("#c4785a") } else { rgb("#ffb48a") }
#let bg = if light {
  gradient.linear(rgb("#ffe6d8"), rgb("#ffe4d2"), rgb("#fff0e4"), rgb("#fff8f1"), angle: 0deg)
} else {
  gradient.linear(rgb("#2c0907"), rgb("#6b150d"), rgb("#b3300f"), rgb("#e8661a"), angle: 0deg)
}
#let wire-c = if light { rgb("#e5502a") } else { rgb("#ffd9bd") }
#let src-fill = if light { white } else { white.transparentize(88%) }
#let src-stroke = if light { rgb("#e5502a") } else { white.transparentize(50%) }
#let src-text = if light { rgb("#3a0d06") } else { white }
#let dst-fill = if light { rgb("#e5502a") } else { rgb("#ffc08a") }
#let dst-text = if light { white } else { rgb("#3a0d06") }
#let node-fill = if light { white } else { rgb("#240706").transparentize(25%) }
#let hex-c = if light { rgb("#e5502a").transparentize(80%) } else { white.transparentize(86%) }

#let mono = "JetBrains Mono"

#set page(width: W + 2 * m, height: H + 2 * m, margin: 0pt, fill: none)
#set text(font: "Inter", fallback: true)

// ── Helpers (all coordinates in pt, relative to the card) ───────────────
#let P(x, y) = (x * 1pt, y * 1pt)
#let seg(a, b) = place(top + left, line(start: P(..a), end: P(..b), stroke: 1.3pt + wire-c))
#let node(x, y) = place(top + left, dx: (x - 3.5) * 1pt, dy: (y - 3.5) * 1pt,
  circle(radius: 3.5pt, fill: wire-c, stroke: none))
#let chip(x, cy, label, dst: false) = place(top + left, dx: x * 1pt, dy: (cy - 17) * 1pt,
  box(width: 112pt, height: 34pt, radius: 7pt,
    fill: if dst { dst-fill } else { src-fill },
    stroke: if dst { none } else { 1pt + src-stroke },
    align(center + horizon, text(font: mono, size: 13pt, weight: "medium",
      fill: if dst { dst-text } else { src-text }, label))))
// Orthogonal "step" wire: horizontal → vertical at mx → horizontal
#let step(x1, y1, mx, x2, y2) = {
  seg((x1, y1), (mx, y1))
  seg((mx, y1), (mx, y2))
  seg((mx, y2), (x2, y2))
}

// ── Layout numbers ──────────────────────────────────────────────────────
#let cys = (55, 105, 155, 205)
#let ports = (82, 114, 146, 178)
#let src-names = ("angle", "color", "datetime", "length")
#let dst-names = ("Angle", "Color", "DateTime", "Length")
#let in-mx = (806, 822, 822, 806)
#let out-mx = (1010, 1026, 1026, 1010)
#let src-x = 660
#let dst-x = 1068
#let enc-x = 885
#let enc-w = 82

// pseudo CBOR bytes for the faint backdrop
#let bytes = ("a3", "65", "61", "6e", "67", "6c", "65", "f9", "56", "40", "65", "63", "6f", "6c", "6f", "72",
  "83", "18", "ff", "18", "7f", "00", "68", "64", "61", "74", "65", "74", "69", "6d", "65", "c1", "1a",
  "65", "4f", "2a", "80", "66", "6c", "65", "6e", "67", "74", "68", "f9", "49", "00", "a1", "64", "74", "79", "70", "65")

#pad(m, block(width: W, height: H, radius: 16pt, clip: true, fill: bg, {
  // faint CBOR hex backdrop
  for r in range(0, 17) {
    place(top + left, dx: 640pt, dy: (6 + r * 15.5) * 1pt,
      text(font: mono, size: 11pt, fill: hex-c, tracking: 0.5pt,
        range(0, 26).map(i => bytes.at(calc.rem(r * 7 + i, bytes.len()))).join(" ")))
  }
  // fade backdrop into the left half
  place(top + left, rect(width: 900pt, height: H, stroke: none, fill: gradient.linear(
    (bg.sample(0%), 0%), (bg.sample(0%), 45%), (bg.sample(0%).transparentize(100%), 100%), angle: 0deg)))

  // wires: Typst values → encoder → Rust types
  for i in range(0, 4) {
    step(src-x + 112, cys.at(i), in-mx.at(i), enc-x, ports.at(i))
    step(enc-x + enc-w, ports.at(i), out-mx.at(i), dst-x, cys.at(i))
    node(src-x + 112, cys.at(i)); node(enc-x, ports.at(i))
    node(enc-x + enc-w, ports.at(i)); node(dst-x, cys.at(i))
  }
  // encoder node
  place(top + left, dx: enc-x * 1pt, dy: 62pt,
    box(width: enc-w * 1pt, height: 136pt, radius: 10pt, fill: node-fill, stroke: 1.3pt + wire-c,
      align(center + horizon, stack(spacing: 5pt,
        text(size: 16pt, weight: "bold", fill: ink, tracking: 0.3pt)[CBOR],
        text(font: mono, size: 9pt, fill: tag)[0xa3 …]))))
  for i in range(0, 4) {
    chip(src-x, cys.at(i), src-names.at(i))
    chip(dst-x, cys.at(i), dst-names.at(i), dst: true)
  }

  // text
  place(top + left, dx: 48pt, dy: 42pt, text(size: 16pt, weight: "medium", fill: tag)[Every Typst value, straight into Rust])
  place(bottom + left, dx: 48pt, dy: -46pt, stack(spacing: 10pt,
    text(size: 82pt, weight: "extrabold", fill: ink, tracking: -1.5pt, top-edge: "ascender", bottom-edge: "descender")[typwire],
    text(size: 24pt, weight: "bold", fill: sub)[Typed data exchange for Typst plugins],
  ))
}))
