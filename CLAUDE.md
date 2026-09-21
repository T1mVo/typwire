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

- `test.typ` — imports `../common.typ` (shared `roundtrip` helper + the
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

`encode` is a single `cbor4ii::serde::to_vec` pass. `decode` is a single parse
pass through cbor4ii's own `Deserializer`, wrapped in a private
`LenientDeserializer` (`wire.rs`). There is no separate validation pass: the
CBOR subset Typst's own `cbor.encode` can produce (int/float/bytes/text/
bool/none/array/map, string-keyed maps, no tags — verified against Typst's
upstream `cbor.rs`, which is itself `ciborium`-based) is exactly what
`Typwire`'s blanket impls admit on the Rust side too (`i64` only, no wider
integer; `BTreeMap`/`HashMap` only over `String` keys), so a well-typed
`Typwire` value can't represent anything `validate` used to reject. Malformed
input (a bypassed-API caller feeding out-of-range integers, non-string map
keys, or CBOR tags directly) still fails closed: cbor4ii's own typed integer
decode uses a checked `TryFrom` internally, mismatched map key types fail
their own typed decode, and an unexpected tag byte simply won't match the
marker any concrete type's decode expects. Failures just surface as a generic
`Error::Deserialization` instead of a dedicated variant.

`LenientDeserializer` exists solely to fix wire width: cbor4ii's *typed*
decode (`deserialize_f32`/`deserialize_f64`, what a derived `Deserialize` uses
for concrete float fields) requires an exact marker-byte match and rejects a
CBOR float encoded narrower than the target width, but Typst's canonical
(narrowest-exact-width) float encoding routinely produces exactly that. Only
`deserialize_f32`/`deserialize_f64` are redirected — to cbor4ii's own
`deserialize_any`, which dispatches per-actual-width correctly and is what
`Value`'s deserialization already relies on — passing the original visitor
through unwrapped, since serde's built-in float visitors already implement
`visit_f32` with widening. Every other type-hinted method keeps cbor4ii's
normal fast path. Because cbor4ii's own `SeqAccess`/`MapAccess`/`EnumAccess`
hold a direct reference to its concrete inner deserializer (bypassing any
outer wrapper that only intercepts the top-level call), reaching nested float
fields requires wrapping the `DeserializeSeed` at every recursion point, not
just the deserializer — the same "deserializer middleware" pattern
`serde_path_to_error` uses. `deserialize_enum` is part of this chain because
`Color` and `Gradient` are internally tagged enums
(`#[serde(tag = "typwire-type")]`) with `f64` fields inside their variants.
`Bytes` serializes as a CBOR byte string (→ Typst `bytes`) while `Vec<u8>`
stays an array. The crate needs the `half-f16` feature for the same width
reason: without it, cbor4ii can't even parse an incoming half-precision float.

### Native Typst value representation

Typst types with no CBOR primitive (`Length`, `Angle`, `Ratio`, `Color` and its
8 spaces, `Gradient` variants, `DateTime`, `Duration`, `Version`, `Type`,
`Center`, `Radius`, `Stop`, `Auto`, `Path`, `Symbol`) are represented as
**tagged dictionaries**: an ordinary map carrying a `typwire-type` string
field (kebab-case, e.g. `"length"`, `"color-oklch"`, `"gradient-linear"`,
`"auto"`, `"path"`, `"symbol"`).

Both sides are organized the same way, by Typst's own reference categories
(<https://typst.app/docs/reference/>) — `foundations/`, `layout/`,
`visualize/` — mirroring how `tests/` is laid out (see *Testing* below):

- Rust: `src/core/crates/typwire/src/{foundations,layout,visualize}/mod.rs`
  are thin re-export modules (`mod <type>; pub use <type>::TypeName;` for
  each type in that category); the actual implementation lives alongside
  each `mod.rs`, in `{foundations,layout,visualize}/<type>.rs`. `color`'s own
  submodule (`color::BLACK` etc.) is `pub` inside `visualize/mod.rs`
  specifically so `lib.rs` can re-export it at the crate root as
  `typwire::color`; every other submodule stays private, with only its types
  re-exported. Anything without a matching reference category (`Any`,
  `OneOfN`, the wire protocol itself) stays at the crate root instead of in
  one of these three modules.
  Each `<type>.rs` defines a private `…Cbor` shadow struct with
  `#[serde(try_from = "…Cbor", into = "…Cbor")]` on the public type;
  `const TYPE_NAME` is the tag, `TryFrom` verifies it.
- Typst: `src/cbor/types/{foundations,layout,visualize}/<type>.typ` each
  expose `type-name`, `encode`, `decode` (`Center` and `Stop` live under
  `src/cbor/types/visualize/gradient/`, since they're only ever nested inside
  a gradient, never a standalone Typst value). `src/cbor/encode.typ` and
  `decode.typ` themselves stay at `src/cbor/`, outside that category split,
  since they aren't type-specific. `encode.typ::encode-inner` dispatches on
  `std.type(...)` when serializing — except `Auto`, which has no `std.type`
  of its own and is matched by direct value equality (`content == auto`)
  instead, same as how `none` is detected. `decode.typ::decode-tagged`
  dispatches on the `typwire-type` string when deserializing. Color/gradient
  use a `common-type-name` prefix match. Note `decode-tagged` evaluates
  `<module>.type-name` for the modules *before* the matching one, so every
  module in that chain must actually define `type-name` (a missing binding is a
  hard error for unrelated types, not just its own).

`auto` and `none` are both Typst keyword literals, not ordinary identifiers —
a bare `auto`/`none` always evaluates to that literal, so
`types/foundations/auto.typ` can't be `#import`ed under its own name (that
binding would be unreachable, shadowed by the keyword in every expression
position). It's imported as `auto-value` in both `encode.typ` and
`decode.typ`. `path` (the type/constructor) is an ordinary stdlib binding
like `int`/`color`, not a keyword, so `types/foundations/path.typ` imports
under its own name fine — code that needs the *builtin* despite that local
shadow (the `std.type(content) == std.path` check in `encode.typ`) uses the
`std.path` qualified form instead, the same trick already used for `type`
(`std.type(content) == std.type`, next to the shadowing `type.encode(...)`).

`path` has no built-in conversion to a plain string (`str(path(...))` is a
type error, and it exposes no accessor) — the only textual form available at
all is `repr()`, which always renders as the Typst source expression
`path("...")`. `path.typ::encode` strips that wrapper
(`repr(value).slice(5, -1)`) and reverses `repr`'s string escaping by hand
via `utils.typ::unescape-repr-string` (backslash/quote/`\n`/`\t`/`\r`/
`\u{...}`, confirmed against Typst's actual escape set empirically since it
isn't fully spelled out in the docs) — deliberately *not* `eval`, since that
would execute the string as Typst code; this is pure string manipulation
instead. `symbol.typ` needs the same trick (see below) and shares it via
`utils.typ::unescape-repr-content`, which `unescape-repr-string` itself is a
thin `.slice(1, -1)` wrapper around.

`Symbol` carries a whole table of named modifier variants (e.g.
`sym.arrow.r` also covers `.double`, `.long`, `.squiggly`, ...), and none of
that table is reachable from script through any dedicated API — `Symbol`'s
`#[scope]` in Typst's own source only exposes the constructor; `variants()`/
`modifiers()` are plain `impl Symbol` methods, never `#[scope]`d (confirmed
by reading `typst-library/src/foundations/symbol.rs` upstream). `repr()`
*does* print the full table, though, as Typst source
(`symbol("→", ("double", "⇒"), ...)`), so `symbol.typ::encode` parses that
text back out with `variant-pattern`, a regex matching the
`("modifiers", "glyph")` tuples — a small, well-defined grammar, not
`eval`. The bare/default entry is deliberately *not* read from that parse;
`encode` gets it from `str(value)` instead (documented, stable API) so the
wire format's most important piece doesn't depend on `repr`'s formatting.
Reconstructing via `symbol(default, ..variants)` in decode is functionally
equivalent to the original (`str(...)` and modifier field access both
agree), but is never `==` to a *built-in* (`sym.*`/`emoji.*`) value no
matter how completely the variant table is restored — confirmed
empirically: Typst represents predefined symbols and anything
`symbol(...)` constructs as different internal enum variants, and derived
`PartialEq` never crosses that boundary. A symbol built with `symbol(...)`
in the first place (not one of Typst's own predefined ones) *does*
round-trip `==`-exact, as long as its variants come back in the same order
`repr` listed them — which they always do, since both `repr` and `decode`
walk the variant list in the same order (equality between two
`symbol(...)`-built values turned out to be order-sensitive too, confirmed
empirically). See `tests/foundations/symbol/test.typ` for both cases.

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
