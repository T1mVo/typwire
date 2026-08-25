// #import "@preview/typwire:0.2.0"
#import "../../src-typst/src/lib.typ" as typwire

#let plugin = plugin("custom_plugin.wasm")

#let report = typwire.call(
  plugin.measure,
  42,
  (
    label: "Value: ",
    scale: 1.5,
  ),
  12pt,
  (
    source: "example",
    enabled: true,
  ),
)

#report.text has width #report.width.
