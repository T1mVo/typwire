#import "../../src-typst/src/lib.typ" as typwire

#let values = (
  90deg,
  12pt,
  25%,
  red,
  gradient.linear(red, blue),
  datetime(year: 2026, month: 8, day: 25),
  duration(hours: 2),
  version(1, 2, 3),
  int,
  content,
  function,
)

#for value in values {
  let decoded = typwire.decode(typwire.encode(value))
  assert(
    decoded == value or repr(decoded) == repr(value),
    message: "roundtrip failed for " + repr(value) + ": " + repr(decoded),
  )
}
