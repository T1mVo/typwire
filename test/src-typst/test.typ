#import "../../src-typst/src/lib.typ" as typwire

#let plugin = plugin("typwire_test.wasm")

#let report = typwire.call(
  plugin.measure,
  42,
  (
    label: "Value: ",
    scale: 1.5,
  ),
  12pt,
  (
    source: "test",
    enabled: true,
    nested: (90deg, 25%, red),
  ),
)

#assert(type(report) == dictionary)
#assert(type(report.width) == length)
#assert(report.width == 12pt)
#assert(report.text == "Value: 42")
#assert(report.metadata.source == "test")
#assert(report.metadata.enabled)
#assert(report.metadata.nested.at(0) == 90deg)
#assert(report.metadata.nested.at(1) == 25%)
#assert(report.metadata.nested.at(2) == red)

#let echoed = typwire.call(
  plugin.echo,
  (
    angle: 45deg,
    length: 3cm,
    ratio: 40%,
    color: color.oklab(50%, 0.0, 0.0),
    datetime: datetime(year: 2026, month: 8, day: 25),
    duration: duration(hours: 2),
    version: version(1, 2, 3),
    type: int,
  ),
)

#assert(echoed.angle == 45deg)
#assert(echoed.length == 3cm)
#assert(echoed.ratio == 40%)
#assert(type(echoed.color) == color)
#assert(echoed.datetime.year() == 2026)
#assert(echoed.duration.hours() == 2)
#assert(echoed.version == version(1, 2, 3))
#assert(echoed.type == int)
