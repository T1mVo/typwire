// Shared helpers for the roundtrip suite. This file is not a test itself
// (Tytanic only discovers directories containing a `test.typ`).

#import "../src/lib.typ" as typwire

/// The compiled test plugin. Built and placed here by `scripts/test.py`;
/// running the suite any other way requires building it first.
#let plugin = plugin("../target/typwire-tests/plugin.wasm")

/// Assert that `value` survives a Typst -> Rust -> Typst round trip through the
/// plugin function `func`.
///
/// Accepts either native equality or matching `repr`, because some Typst values
/// (gradients, some colors) do not compare structurally, and dictionaries
/// re-serialized from Rust come back with sorted keys.
#let roundtrip(func, value) = {
  let got = typwire.call(func, value)
  assert(
    got == value or repr(got) == repr(value),
    message: "round trip mismatch:\n  in:  " + repr(value) + "\n  out: " + repr(got),
  )
}
