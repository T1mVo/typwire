# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`typwire` is a two-sided library for typed, Serde-backed data exchange across the
Typst ⇆ WASM-plugin boundary. It ships as **two coordinated packages that must
stay version-locked** (`0.2.0` today, set in both `typst.toml` and every
`Cargo.toml`):

- a **Typst package** (`src/lib.typ` + `src/cbor/`) that encodes native Typst
  values to CBOR and reconstructs them on the way back;
- a **Rust crate** (`src/core/crates/typwire`) plus its proc-macro crate
  (`src/core/crates/typwire-macros`) that a plugin author depends on to decode
  those bytes into owned Rust types and re-encode results.

The version guard in `typwire-macros` is only implicit — mismatched packages
fail at the wire level, not at compile time. When bumping the version, change it
in `typst.toml`, `src/core/Cargo.toml` (`workspace.dependencies`), all crate
`Cargo.toml`s under `src/core/crates/`, and `test/`/`example/` manifests
together.

## Testing

The real test suite is the **Typst roundtrip suite** under `tests/`, run with
[Tytanic](https://typst-community.github.io/tytanic/) (`tt`). Each
`tests/<type>/` directory is a `compile-only` test with two files:

- `test.typ` — imports `/tests/roundtrip.typ` (shared `check` helper + the
  `plugin(...)` handle) and asserts that Typst values survive a
  Typst→Rust→Typst round trip via `typwire.call`.
- `plugin.rs` — usually one `#[typwire::export] fn roundtrip_<type>(v: T) ->
  Result<T, String>` for that test (`tests/rename/` covers the `rename` option
  and defines several).

Tytanic has **no build hook**, so `scripts/test.py` is the entry point: it
`cargo build`s `typwire-tests-plugin` for wasm, copies the artifact to
`target/typwire-tests/plugin.wasm` (git-ignored; the path every `test.typ`
loads), then runs `tt`.

```sh
python3 scripts/test.py               # build plugin + tt run --no-fail-fast (all tests)
python3 scripts/test.py length ratio  # forwarded: tt run length ratio
python3 scripts/test.py --no-build type   # skip cargo, reuse existing wasm
python3 scripts/test.py update        # forwarded: tt update
```

`tests/<type>/plugin.rs` files are **`include!`d into the crate root** of
`src/core/crates/typwire-tests-plugin/src/lib.rs` (a `mod` won't work — the
`#[wasm_func]` wrapper needs `initiate_protocol!()`'s items in scope, which only
holds at the crate root). Consequences: a case file must have **no `use`
items** (they collide when concatenated — rely on `lib.rs`'s glob imports or
fully-qualified paths) and each exported fn name must be **globally unique**
(hence `roundtrip_<type>`). Adding a test = new `tests/<name>/` dir + one
`case!("<name>");` line in that `lib.rs`.

`typwire-tests-plugin` is a wasm-only cdylib (it can't link for the host), so it
is kept out of the workspace's `default-members`; a bare `cargo build`/`cargo
test` in `src/core` skips it.

## Repository layout note (mid-refactor)

Commit `ff6be1127` ("refactor: change repo layout") moved `src-typst/src/` →
`src/` and the Rust sources → `src/core/crates/…`, **but did not update the
downstream references**. The `tests/` + `scripts/test.py` path (see *Testing*)
is current and green; the older harness is stale and will fail:

- `scripts/{test,example,package,clean}.sh` reference `src-typst/` paths that no
  longer exist. `scripts/test.sh` is superseded by `scripts/test.py`.
- `test/src-typst/*.typ` and `example/src-typst/*.typ` import
  `../../src-typst/src/lib.typ` (should be the new `src/lib.typ`).
- `test/Cargo.toml` and `example/Cargo.toml` use `typwire = { path = ".." }`,
  which now points at the repo root (no `Cargo.toml` there) instead of
  `src/core/crates/typwire`.

The Rust workspace itself builds and tests cleanly.

## Commands

Rust workspace lives in `src/core/` (run from there, or pass `--manifest-path`):

```sh
cd src/core
cargo test                       # unit tests + doctests for typwire + typwire-macros
cargo test -p typwire            # single crate
cargo test --doc -p typwire      # doctests only (type modules carry runnable examples)
cargo test bytes_use_a_byte_string   # single test by name
cargo build --target wasm32-unknown-unknown   # plugin-target build
```

Note: `src/core/Cargo.toml` declares `[workspace.dev-dependencies]` (trybuild),
which Cargo ignores with a warning — the UI/compile-fail tests it fed
(`tests/macros.rs`, `tests/ui/**`) have been deleted and not yet re-wired.

End-to-end Typst tests: see *Testing* above (`python3 scripts/test.py`).

`scripts/package.sh` assembles the publishable Typst package under
`scripts/typwire/<version>/` (also references stale `src-typst/` paths).

Typst debug assertions in `src/cbor/**` are gated on
`--input debug-assert=true` (see `is-debug-assert` in `src/cbor/utils.typ`); the
roundtrip suite does not enable them.

The working copy is managed with **Jujutsu** (`.jj/`) on top of Git; plain `git`
commands still work.

## Architecture

### The wire contract

`Typwire` (in `src/core/crates/typwire/src/wire.rs`) is a marker trait:
`Serialize + DeserializeOwned` plus explicit opt-in. Deriving `Serialize`/
`Deserialize` is **not** enough to cross the boundary — a type must also derive
`Typwire` (user types) or have a blanket/manual impl (primitives, collections,
native wrappers). This deliberately prevents an unrelated Serde type from
silently becoming part of a plugin ABI.

`encode`/`decode` (`wire.rs`) route everything through `cbor4ii::core::Value`
(the `serde1` feature) and a `validate` pass that enforces Typst's reliable
CBOR subset: integers must fit `i64` and map keys must be strings. CBOR tags
are never observed as a literal `Value::Tag` in practice: cbor4ii's `Value`
decoder silently collapses the bignum tags (2/3) into `Value::Integer` when
they fit `i128` (still caught by `validate`'s ordinary integer range check,
surfacing as `IntegerOutOfRange`) and hard-errors before producing a `Value` at
all for anything else — an out-of-`i128` bignum or any non-bignum tag — which
`encode`/`decode` surface as `MalformedCbor` rather than
`UnsupportedWireShape`. The `Value::Tag` match arms in `validate` are therefore
dead code today, kept only because the enum is `#[non_exhaustive]` and as a
safety net if that decoding behavior ever changes. `Bytes` serializes as a CBOR
byte string (→ Typst `bytes`) while `Vec<u8>` stays an array.

cbor4ii's *typed* decode (what a derived `Deserialize` uses for concrete
fields, e.g. `deserialize_f64`) is strict about wire width — it rejects a CBOR
float encoded with fewer than 8 bytes even when the target is `f64`, so
Typst's canonical (narrowest-exact-width) float encoding fails a raw
`from_slice::<T>`. `decode` works around this by decoding into `Value` first
(whose deserialization is width-tolerant) and going through
`value_to_owned`, which re-serializes that `Value` — cbor4ii always writes
floats at full width — before the final typed decode. This costs a second
parse pass but is necessary for correctness, not just validation; see
`wire.rs`'s doc comment on `decode`. The crate needs the `half-f16` feature
for the same reason: without it, cbor4ii's `Value` decoder cannot even parse
an incoming half-precision float at all.

### Native Typst value representation

Typst types with no CBOR primitive (`Length`, `Angle`, `Ratio`, `Color` and its
8 spaces, `Gradient` variants, `DateTime`, `Duration`, `Version`, `Type`,
`Center`, `Radius`, `Stop`) are represented as **tagged dictionaries**: an
ordinary map carrying a `typwire-type` string field (kebab-case, e.g.
`"length"`, `"color-oklch"`, `"gradient-linear"`).

This tag scheme is implemented **twice and must agree**:

- Rust: each `src/core/crates/typwire/src/<type>.rs` defines a private
  `…Cbor` shadow struct with `#[serde(try_from = "…Cbor", into = "…Cbor")]` on
  the public type. `const TYPE_NAME` is the tag; `TryFrom` verifies it.
- Typst: each `src/cbor/<type>.typ` exposes `type-name`, `encode`, `decode`.
  `src/cbor/cbor.typ::encode-inner` dispatches on `std.type(...)` when
  serializing; `src/cbor/decode.typ::decode-tagged` dispatches on the
  `typwire-type` string when deserializing. Color/gradient use a
  `common-type-name` prefix match. Note `decode-tagged` evaluates
  `<module>.type-name` for the modules *before* the matching one, so every
  module in that chain must actually define `type-name` (a missing binding is a
  hard error for unrelated types, not just its own).

Adding or renaming a native type means editing both sides plus the tag lists in
`any.rs` (`from_map`) and `src/cbor/decode.typ`.

### `Any` and `OneOfN`

- `Any` (`any.rs`) is a hand-written Serde enum for dynamic values. Its
  `Deserialize` goes through `cbor4ii::core::Value` and **tries the native tag
  before falling back to a plain dictionary**, so a `Length` inside an `Any`
  round-trips as `Any::Length`, not `Any::Dictionary`.
- `OneOf2`..`OneOf8` (`one_of.rs`, macro-generated) are `#[serde(untagged)]`
  enums. Variants are tried **in declaration order** — overlapping
  representations resolve to the first match.

### The `#[typwire::export]` macro

In `typwire-macros/src/lib.rs`. It takes a free function
`fn f(a: A, b: B, …) -> Result<T, E>` and generates:

1. a renamed, hash-suffixed copy of the original body
   (`__typwire_export_impl_f_<hash>`), and
2. a `#[wasm_func]` wrapper `fn f(a: &[u8], b: &[u8], …) -> Result<Vec<u8>,
   String>` that decodes each argument via `__private::decode_argument`
   (errors name the function and argument), calls the impl, maps `E` through
   `Display`, and encodes the result.

`#[typwire::export(rename = "wire-name")]` parses one option (`syn::meta::parser`
in the `export` entry point). Since `#[wasm_func]` always exports under the
wrapper's *identifier*, the rename path skips `#[wasm_func]` entirely: it emits
an `extern "C"` fn with `#[unsafe(export_name = "wire-name")]` that re-declares
the two `typst_env` protocol imports, reads/splits the argument buffer itself,
and calls the same decode/encode body. So `rename` accepts arbitrary strings
(kebab-case included); the no-rename path is byte-for-byte unchanged. Typst
reaches a non-identifier export via `dictionary(plugin).at("wire-name")`.

`validate_signature` rejects `const`/`async`/`unsafe`/`extern`/variadic/generic
functions, receivers/methods, non-identifier or `ref` argument patterns, and
**any reference type anywhere in an argument type** (`first_reference_span`
walks the type with a `syn` visitor). The return type must be a literal
`Result<_, _>` with two type args. `cfg`/`cfg_attr` attributes are forwarded to
both the impl and the wrapper; `wasm_func` is stripped from the author's attrs.

The derive macro (`derive_typwire`) just emits `impl Typwire for T {}` with a
`where` bound requiring `Serialize + DeserializeOwned`. `typwire_path()` uses
`proc-macro-crate` to resolve the crate name, and `lib.rs` has
`extern crate self as typwire;` so the macros work inside the crate's own tests.

### Typst entrypoint

`src/lib.typ` re-exports `cbor` as `encode`/`decode` and defines
`typwire.call(function, ..args)`: rejects named args, encodes each positional
arg **independently**, calls the plugin function with the resulting byte
strings, and decodes the single returned byte string. This positional,
separately-encoded calling convention is what `#[typwire::export]`'s
`&[u8]`-per-parameter wrapper expects.

## Explicitly unsupported (by design)

Borrowed values (`&str`, `&[u8]`, borrowed models), Typst `content`, `function`,
fractions, directions, relative lengths, and `decimal`. `content`/`function`
encoding panics on the Typst side; references are a compile error via the macro.
