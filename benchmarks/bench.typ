#import "../src/lib.typ" as typwire
#import "@preview/sertyp:0.1.5" as sertyp

#let workload = sys.inputs.at("workload")
#let lib = sys.inputs.at("lib")
#let mode = sys.inputs.at("mode", default: "any")
#let verify = sys.inputs.at("verify", default: "false") == "true"

#let make-objects(n) = range(n).map(i => (
  x: i,
  y: i * 2,
  label: "item-" + str(i),
))
#let make-natives(n) = range(n).map(i => (
  (i * 1pt + 0.5pt),
  calc.rem(i, 360) * 1deg,
  calc.rem(i, 100) * 1%,
  rgb(calc.rem(i, 256), calc.rem(i * 7, 256), 128),
))

#let workloads = (
  // Fixed cost: plugin load + one trivial call.
  noop: () => (0,),
  // One large flat array of ints.
  ints: () => (range(6000),),
  // One long string.
  string: () => ("typst " * 20000,),
  // One large byte payload.
  bytes: () => (bytes(range(40000).map(i => calc.rem(i, 256))),),
  // Many small dictionaries (`tests/stress/many-objects`).
  objects: () => (make-objects(1000),),
  objects-large: () => (make-objects(6000),),
  // Deeply nested arrays/dictionaries.
  nested: () => {
    let tree(depth, i) = if depth == 0 { i } else {
      (id: i, kids: range(3).map(k => tree(depth - 1, i * 3 + k)))
    }
    (tree(7, 0),)
  },
  // Typst-native values with no CBOR primitive: lengths, angles, ratios, colors.
  natives: () => (make-natives(300),),
  natives-large: () => (make-natives(1500),),
  // Many distinct calls with a small payload each (per-call overhead).
  calls-small: () => range(2000).map(i => (
    x: i,
    y: i * 2,
    label: "item-" + str(i),
  )),
  // Many distinct calls with a gradient each.
  calls-gradient: () => range(300).map(i => gradient.linear(
    ..color.map.viridis,
    angle: calc.rem(i, 100000) * 1deg,
  )),
)

#let typed-fn = (
  ints: "ints",
  string: "string",
  bytes: "bytes",
  objects: "objects",
  objects-large: "objects",
  calls-small: "object",
  natives: "natives",
  natives-large: "natives",
  calls-gradient: "gradient",
)
#let fn-name = if mode == "typed" { typed-fn.at(workload) } else { "bench" }

#let call = if lib == "typwire" {
  let p = plugin("/benchmarks/target/typwire.wasm")
  v => typwire.cbor.call(dictionary(p).at(fn-name), v)
} else if lib == "sertyp" {
  let p = plugin("/benchmarks/target/sertyp.wasm")
  v => sertyp.call(dictionary(p).at(fn-name), v)
}

#let calls = workloads.at(workload)()

#if lib != "none" {
  for v in calls {
    let out = call(v)
    if verify {
      // Gradients/some colors don't compare structurally; fall back to `repr`.
      if out != v and repr(out) != repr(v) {
        panic("round trip mismatch for " + workload)
      }
    }
  }
}
