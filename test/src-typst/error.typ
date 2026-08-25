#import "../../src-typst/src/lib.typ" as typwire

#let plugin = plugin("typwire_test.wasm")

#typwire.call(plugin.fail, "expected Rust failure")
