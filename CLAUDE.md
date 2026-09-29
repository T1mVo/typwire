# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`typwire` is a two-sided library for typed, Serde-backed data exchange across the
Typst ⇆ WASM-plugin boundary. It ships as **two coordinated packages that must
stay version-locked** (`0.3.0` today, set in both `typst.toml` and every
`Cargo.toml`):

- a **Typst package** (`src/lib.typ` + `src/wire/` + `src/format/`) that
  encodes native Typst values to a wire format and reconstructs them on the
  way back;
- a **Rust crate** (`src/core/crates/typwire`) plus its proc-macro crate
  (`src/core/crates/typwire-macros`) that a plugin author depends on to decode
  those bytes into owned Rust types and re-encode results.

Four wire formats are supported — CBOR (the default on both sides), JSON,
YAML, TOML — see *Wire formats* below.

The version guard in `typwire-macros` is only implicit — mismatched packages
fail at the wire level, not at compile time. When bumping the version, change it
in `typst.toml`, `src/core/Cargo.toml` (`workspace.dependencies`), all crate
`Cargo.toml`s under `src/core/crates/`, and `test/`/`example/` manifests
together.

## Testing

The real test suite is the **Typst roundtrip suite** under `tests/`, run with
[Tytanic](https://typst-community.github.io/tytanic/) (`tt`). Every test
covers all four wire formats: `tests/<type>/{cbor,json,yaml,toml}/`, each a
`compile-only` test with two files:

- `test.typ` — imports `../../common.typ` (shared `roundtrip-format` helper
  + the `plugin(...)` handle) and asserts that Typst values survive a
  Typst→Rust→Typst round trip via `typwire.<format>.call`. Every export is
  reached through direct kebab-case field access,
  `plugin.roundtrip-<type>-<format>` — confirmed empirically that this
  works even though `-` is the subtraction operator elsewhere: Typst's
  tokenizer only treats a bare `-` as subtraction when it's set off by
  whitespace (`a - b`); directly against identifier characters with no
  space (`a-b`, and so `plugin.roundtrip-i64-json`), it continues the same
  identifier/field-name token instead. An earlier version of this suite
  assumed otherwise and routed every export through
  `dictionary(plugin).at("...")`, which is only actually required for a
  `rename` string that isn't identifier-shaped at all (contains a space,
  starts with a digit, ...) — `tests/rename/` is the one place that still
  uses it, and only for a dictionary membership check (`name not in
  exports`), not to look up a function.
- `plugin.rs` — usually one function per format,
  `#[typwire::export(format = "<format>", rename = "roundtrip-<type>-<format>")]
  fn roundtrip_<type>_<format>(v: T) -> Result<T, String>` (`format`/the Rust
  fn-name suffix omitted for `cbor`'s default, though `rename` is still given
  — uniform accessor pattern across all four beats saving one macro arg).
  `tests/rename/` combines this with a *second*, orthogonal rename (its own
  point is exercising `rename` itself) by suffixing its existing custom
  names, e.g. `"round-trip-json"`. A case is free to define several
  functions when closely related (`tests/foundations/regex/*/plugin.rs`
  each have two).

Tytanic has **no build hook**, so `scripts/test.py` is the entry point: it
`cargo build`s `typwire-tests-plugin` for wasm, copies the artifact to
`target/typwire-tests/plugin.wasm` (git-ignored; the path every `test.typ`
loads), then runs `tt`. `--release` layers on a few independent size levers,
each verified against the real 220-test suite (not just "it builds") before
being kept, and each measured in isolation by toggling it with the others
held fixed rather than trusting a single combined before/after:

- **`src/core/.cargo/config.toml`** sets `-C
  target-feature=+bulk-memory,+simd128` for the `wasm32-unknown-unknown`
  target only (`[target.<triple>]` tables don't leak into host builds of
  `typwire`/`typwire-macros` in the same workspace). Confirmed safe first:
  Typst's plugin runtime is `wasmi` (a pure-Rust interpreter,
  `typst-library`'s dependency), whose `Config` has both proposals on by
  default — only *relaxed* SIMD is explicitly turned off in Typst's own
  `Plugin::new` — so a wasm module using them loads and runs there, unlike
  most other wasm hosts where this would need checking first. Cargo
  discovers `.cargo/config.toml` by walking up from the process's *current
  working directory*, not from `--manifest-path` — confirmed the hard way,
  the flag silently wasn't applied (identical artifact, unchanged mtime)
  until `build_plugin` passed `cwd=CARGO_MANIFEST.parent` to
  `subprocess.run`. Effect: raw artifact 2,939 KiB → 2,875 KiB (−2.2%), no
  change to `tt run`'s own time.
- **`[profile.release]`** in `src/core/Cargo.toml` is `lto = true,
  codegen-units = 1, strip = true` — raw artifact 2,875 KiB → 2,622 KiB
  (−8.8% more), `tt run` unchanged (~5.8s). `opt-level = "z"` and
  `panic = "abort"` were both tried here too and then removed:
  `opt-level = "z"` was by far the biggest single lever in this whole list
  (2,622 KiB → 1,464 KiB, another −44.2%) but was also the *only* setting in
  the entire chain that measurably slowed down actual test *execution*, not
  just build output size — `tt run --no-fail-fast` went from ~5.8s to ~7.8s
  (+34%), confirmed by toggling that one line alone with everything else held
  fixed. `panic = "abort"` made no measurable difference either way for this
  wasm target, which can't unwind regardless of this setting. Both are still
  workspace-wide settings if reintroduced (Cargo cannot scope `lto`/`panic`
  to one package), so a `--release` build of `typwire`/`typwire-macros` for
  the host would inherit them too; the documented commands (`cargo test`,
  `cargo build`) use the untouched `dev` profile and are unaffected either
  way.
- **`talc` as the global allocator** (`typwire-tests-plugin/src/lib.rs`,
  `#[global_allocator] static ALLOCATOR: talc::wasm::WasmDynamicTalc =
  talc::wasm::new_wasm_dynamic_allocator();`, guarded by the same
  `#[cfg(all(not(target_feature = "atomics"), target_family = "wasm"))]`
  talc's own docs use) replaces the default wasm32-unknown-unknown allocator
  (`dlmalloc`, via `std`) — this crate is single-threaded and wasm-only, so
  none of `dlmalloc`'s general-purpose/multi-target design is earning its
  keep here. At the settled profile (no `opt-level = "z"`): raw artifact
  2,622 KiB → 2,321 KiB (**−11.5%**), no measurable change to `tt run`'s time
  in either direction (~5.5–5.8s throughout, run-to-run noise). With
  `opt-level = "z"` reintroduced, the gap nearly vanishes (1,464 KiB → 1,456
  KiB, −0.5%) — aggressive size optimization already squeezes `dlmalloc` down
  close to `talc`'s size, so there's less room left for the allocator choice
  to matter; `opt-level = "z"`'s own +34% slowdown is also unchanged by
  `talc`, confirmed per-format the same way as before (CBOR/JSON/YAML/TOML
  all landed within a few percent of their pre-`talc` numbers). The one real
  cost: **`talc` measurably slows down the plain debug build** specifically
  (~27s → ~31s for the full suite, +~17%) — `talc`'s own code is compiled
  unoptimized like the rest of a `dev`-profile build, while `std`'s
  `dlmalloc` ships pre-built and optimized regardless of the *consuming*
  crate's profile, so swapping it out trades away that free optimization in
  exactly the one profile where nothing here otherwise costs anything.

On top of that, `wasm-opt -O4 --enable-bulk-memory-opt --enable-simd`
(Binaryen, checked via `shutil.which`, not `require`d like `cargo`/`tt` — its
absence falls back to a plain copy rather than erroring) still runs, taking
the final artifact 2,622 KiB → 2,253 KiB (−14.1%). **`--enable-bulk-memory-opt
--enable-simd` are load-bearing, not optional, here** — once
`.cargo/config.toml` makes `+bulk-memory,+simd128` genuinely reach rustc, the
input `.wasm` contains real `memory.copy`/`memory.fill`/`v128.load`/
`v128.store` instructions, and plain `wasm-opt -O4` *without* those two flags
fails validation outright (`unexpected false: memory.copy operations require
bulk memory operations [--enable-bulk-memory-opt]`, `...SIMD operations
require SIMD [--enable-simd]`, dozens of times, no output file written) —
confirmed by actually running it that way, not inferred. (The first time
these two flags were measured, before `.cargo/config.toml` existed, the
artifact genuinely contained neither feature and they really were near-inert
then — that number does not carry over once the build flags are in the mix,
which is exactly why isolating one change at a time instead of trusting a
single combined before/after matters.) `-Oz` instead of plain `-O4` was also
tried and dropped: ~2 more percentage points of size for ~1.4s more
`wasm-opt` time, no change to `tt run`.

Current state (`.cargo/config.toml` + `lto`/`codegen-units`/`strip` + `talc`,
without `opt-level = "z"`/`panic = "abort"`, + `wasm-opt`): 220/220 tests
pass, final artifact **29.9% smaller** than the unoptimized `--release`
baseline this investigation started from (2,939 KiB → 2,059 KiB), `tt run`
unchanged at ~5.5–5.8s throughout in release mode — i.e. all of the size win
above and none of the release-mode speed cost, since `opt-level = "z"` (the
only lever that trades speed for size, and the only one `talc` doesn't
meaningfully help with) isn't in the mix. Re-add `opt-level = "z"` for a
1,305 KiB artifact (55.6% smaller than baseline) at the cost of the known
+34%-slower `tt run`, or accept plain debug builds running ~17% slower for
`talc`'s size win even when nothing else here is in play.

```sh
python3 scripts/test.py               # build plugin + tt run --no-fail-fast (all tests)
python3 scripts/test.py --release     # release profile; also runs wasm-opt if installed
python3 scripts/test.py length ratio  # forwarded: tt run length ratio
python3 scripts/test.py --no-build type   # skip cargo, reuse existing wasm
python3 scripts/test.py update        # forwarded: tt update
```

`tests/<type>/<format>/plugin.rs` files are **`include!`d into the crate
root** of `src/core/crates/typwire-tests-plugin/src/lib.rs` (a `mod` won't
work — the `#[wasm_func]` wrapper needs `initiate_protocol!()`'s items in
scope, which only holds at the crate root). Consequences: a case file must
have **no `use` items** (they collide when concatenated — rely on
`lib.rs`'s glob imports or fully-qualified paths) and each exported fn name
— and any locally defined type, e.g. a `#[derive(Typwire)]` struct — must be
**globally unique** across the whole suite (hence `roundtrip_<type>_<format>`,
and `<Name><Format>` for a per-case struct, e.g.
`tests/foundations/dictionary/struct/*/plugin.rs`'s `PointCbor`/`PointJson`/
etc.). `case!("<name>")` (the same `lib.rs`) expands to all four
`include!`s at once, so adding a test is: `mkdir -p
tests/<name>/{cbor,json,yaml,toml}`, a `test.typ` + `plugin.rs` pair in
each, then one `case!("<name>");` line.

`typwire-tests-plugin` is a wasm-only cdylib (it can't link for the host), so it
is kept out of the workspace's `default-members`; a bare `cargo build`/`cargo
test` in `src/core` skips it. Its `Cargo.toml` enables `json`/`yaml`/`toml`
alongside `decimal`/`regex` unconditionally — one wasm binary covers every
format; the feature only makes the code paths exist, each export still
compiles to its own dedicated wasm function regardless of format.

`tests/stress/{bytes,multi-arg,many-objects}/` are the one group that
deliberately breaks the "a couple of small values" shape every other test
uses — each verifies the wire protocol still round-trips correctly under a
meaningfully large workload, not just a representative one: `bytes` a single
large `Bytes` payload (kept separate from `multi-arg` since JSON/YAML/TOML's
tagged-array `Bytes` rewrite — see *Wire formats* — makes a large payload's
wire *size*, not just its shape, format-sensitive in a way nothing else
here is), `multi-arg` several independently-heavy arguments in one call (a
long string, a many-stop gradient, a large array), and `many-objects` a
large array of small structs (stressing the recursive tree-walk side of
encode/decode — many calls — rather than the raw byte-count side). Payload
sizes (tens of thousands of bytes/array elements) were picked empirically by
timing (`tt run stress/<name>/<format>` against `time`, not guessed), and
deliberately push the full suite's run time up rather than staying
negligible — the point of this group is to actually be heavy. `serde-saphyr`/
`toml` are both meaningfully slower than `cbor4ii`/`serde_json` per element
at this payload shape (YAML came out ~15–20x slower than CBOR for the same
element count here), and **debug vs. release wasm is the bigger swing**:
the full 220-test suite (these 12 plus the other 208, which stay
millisecond-fast throughout) runs in ~27s under a plain `cargo build`
(`scripts/test.py`'s default) but ~6s with `scripts/test.py --release` —
these tests are the reason that gap exists at all, so reach for `--release`
when iterating near them specifically.

`opt-level = "z"`'s +34%-suite-wide slowdown (see *Testing* above) is not
spread evenly across formats — measured per-format by re-running each
format's 55 tests in isolation (`tt run` given just that format's test IDs),
`opt-level = "z"` on vs. off: CBOR ~828ms → ~835ms (+0.8%, noise), JSON
~1,049ms → ~1,209ms (+15.3%), TOML ~1,494ms → ~1,780ms (+19.1%), YAML
~3,242ms → ~4,846ms (**+49.5%**, ~1.6s of the ~2s total regression by
itself). This tracks the same format ordering as everything else in this
file: CBOR is one direct `cbor4ii` pass with no extra allocation; JSON/YAML/
TOML all route through the `value_from_owned`/tree-rewrite round trip (see
*Wire formats*), and YAML additionally hops through `serde_json::Value`
first (the Norway-problem fix) — the more allocation-heavy/branch-heavy a
format's path already is, the more `opt-level = "z"`'s speed-for-size trade
costs it specifically.

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
cargo test -p typwire --features decimal,regex   # also exercise `Decimal`/`Regex`
cargo test -p typwire --features json,yaml,toml  # also exercise the non-default wire formats
```

Note: `src/core/Cargo.toml` declares `[workspace.dev-dependencies]` (trybuild),
which Cargo ignores with a warning — the UI/compile-fail tests it fed
(`tests/macros.rs`, `tests/ui/**`) have been deleted and not yet re-wired.

End-to-end Typst tests: see *Testing* above (`python3 scripts/test.py`).

`scripts/package.sh` assembles the publishable Typst package under
`scripts/typwire/<version>/` (also references stale `src-typst/` paths).

Typst debug assertions in `src/wire/**` are gated on
`--input debug-assert=true` (see `is-debug-assert` in `src/wire/utils.typ`); the
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

This section describes CBOR specifically — `Cbor` (`src/format/cbor.rs`),
the default `Format` impl and the one every native type/`Any`/`OneOfN` was
originally designed against. See *Wire formats* below for `Json`/`Yaml`/
`Toml` and why almost none of what follows needed to change to add them.
The crate's top-level `encode`/`decode` free functions (`wire.rs`) always
mean `Cbor` (`Cbor::encode`/`Cbor::decode` under the hood) — they predate
the `Format` trait and keep their original, unparameterized signature so
existing direct callers see no change.

`encode` is a single `cbor4ii::serde::to_vec` pass. `decode` is a single parse
pass through cbor4ii's own `Deserializer`, wrapped in a private
`LenientDeserializer` (`format/cbor.rs`). There is no separate validation pass: the
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
stays an array — true only for `Cbor`; see *Wire formats* for how the other
three formats represent `Bytes` instead. The crate needs the `half-f16`
feature for the same width reason: without it, cbor4ii can't even parse an
incoming half-precision float.

### Wire formats

`Format` (`wire.rs`) is the trait behind multi-format support: `fn
encode<T: Typwire>(&T) -> Result<Vec<u8>, Error>` / `fn decode<T:
Typwire>(&[u8]) -> Result<T, Error>`. Every impl lives under
`src/format/*.rs`, one file per format (`format/mod.rs` re-exports each
marker type; `cbor` is unconditional, `json`/`yaml`/`toml` each behind
their own feature). `Cbor` (`format/cbor.rs`, unconditional) implements it
with the body described above; `Json`/`Yaml`/`Toml` (`format/{json,yaml,
toml}.rs`, each behind its own crate feature — `json`/`yaml`/`toml`, off by
default, same convention as `decimal`/`regex`) implement it with
`serde_json`/`serde-saphyr`/`toml` respectively. Picking a format whose feature isn't
enabled is a compile error (the marker type doesn't exist), the same
property `Regex`/`Decimal` already have when their feature is off.
`#[typwire::export(format = "json")]` (default `"cbor"`;
`typwire-macros/src/lib.rs`) resolves the option to the matching marker
type and threads it as a generic parameter into `__private::decode_argument`/
`encode_result`. On the Typst side, `typwire.json`/`typwire.yaml`/
`typwire.toml` (`src/format/*.typ`) each wrap the matching native Typst
format function (`json.encode`, ...) around the shared `wire.encode`/
`wire.decode` tree layer (`src/wire/encode.typ`/`decode.typ` — the
`encode-inner`/`decode-inner` from before this feature existed, promoted to
public and renamed since they're no longer CBOR-specific). Unlike the Rust
side's generic-parameter dispatch, the Typst tree layer takes `format` as
an ordinary named string argument, `format: "cbor"` (default `"cbor"`
throughout, not just at the public entrypoints), threaded through every
recursive call, all the way down into every `wire/types/**` module's own
`encode`/`decode` — almost none of which actually branch on it (see
*Format threading* below for why it's there anyway and which two do use
it). `src/lib.typ` builds a `formats` dictionary mapping each
name to its module and exposes generic `encode(value, format: "cbor")`/
`decode(data, format: "cbor")`/`call(function, ..args, format: "cbor")` on
top of it; `typwire.cbor`/`.json`/`.yaml`/`.toml` are the same four
modules re-exported directly (not wrapped in a plain dictionary — Typst
allows `module.field(..)` dot-call sugar but not `dict.field(..)` when
`field` merely holds a function, confirmed the hard way when an earlier
version of this wrapping broke every `typwire.<format>.call` call site),
so `typwire.<format>.encode(value)` and `typwire.encode(value, format:
"<format>")` run the identical function either way. `typwire.debug` (no
serialization, see below) takes the same `format:` argument too, purely to
pick which format's `bytes`/`none` tagging rules to preview.

Two problems anticipated from the start, solved without touching almost any
of the type system described elsewhere in this document — plus a third one
found only once the multi-format test suite existed to catch it (below):

- **Numbers.** JSON/YAML can't distinguish an integer from a float at the
  type-system level the way CBOR/TOML can — but confirmed empirically,
  Typst's own `json.encode`/`yaml.encode` already write `5` for an int and
  `5.0` for a float (never a bare `5` for a whole float), and
  `json(...)`/`yaml(...)` already decode back to `int`/`float` correctly
  based on that literal syntax — so nothing in `typwire.typ` needs to care.
  On the Rust side, `Any`'s `Deserialize` already goes through
  `cbor4ii::core::Value`, and that type's `Deserialize` is fully generic
  over `D: Deserializer<'de>` (not CBOR-specific in what it consumes, only
  in which crate it happens to live in) — `deserialize_any`/Visitor
  dispatch is what every self-describing serde format uses to report
  int-vs-float, so `Value` reconstructs `Any` correctly regardless of which
  format produced the bytes, with **zero changes to `any.rs`**. TOML has no
  ambiguity to begin with: its grammar requires the decimal point for
  floats, and `toml::Value` already has separate variants. (This claim
  holds for int-vs-float specifically — it does *not* generalize to every
  scalar `cbor4ii::core::Value` reads through every format's
  `deserialize_any`, confirmed the hard way; see the `Yaml`-specific bug
  below.)
- **Bytes.** None of JSON/YAML/TOML have a native byte-string type — Typst's
  own `json.encode(bytes(..))`/`yaml.encode`/`toml.encode` all confirmed
  (empirically) to silently produce the useless placeholder string
  `"bytes(3)"` rather than actual content. `Cbor` is untouched — it keeps
  encoding `Bytes`/`bytes` natively on both sides, exactly as described
  above. The other three formats turn a `bytes` leaf into a tagged array
  *inside* the shared core now, not around it, and — unlike every other
  format-sensitive case in this section — `bytes` gets a real
  `wire/types/foundations/bytes.typ` module of its own, the same
  `encode(value, format: "cbor")`/`decode(value, format: "cbor")` shape
  every other native type file has: `format == "cbor"` returns `value`
  unchanged, anything else returns `(typwire-type: "bytes", data:
  array(value))`. `wire/encode.typ::encode` dispatches to it from a
  dedicated `std.type(content) == std.bytes` branch — deliberately **not**
  wrapped in a recursive `encode(...)` call the way every other tagged
  type's dispatch line is (`result = encode(alignment.encode(...), ...)`,
  etc.): `bytes-type.encode` returns the *same* `bytes` value unchanged
  under `"cbor"`, so recursing into `encode` on that result would hit the
  `std.bytes` branch again and loop forever — confirmed by writing it the
  "consistent" way first and watching it hang. Every other type's `encode`
  always returns a *new* dictionary shape instead, which is what makes
  recursing into it both safe and necessary there (to walk that
  dictionary's own fields) but wrong here. `is-primitive` (see below) no
  longer knows anything about `bytes` at all as a result — it used to be
  the one place bytes' format-sensitivity lived, before `bytes.typ` existed
  to own that decision itself. There is no separate tree-walk over the
  already-built tree the way an earlier version of this feature had
  (`wire/rewrite-bytes.typ`, since deleted; `format/json.typ`/`yaml.typ`/
  `toml.typ` used to call it on `wire.encode`'s output before handing the
  tree to their native encoder). Decode is simpler and fully shared,
  unchanged by this refactor — `decode-tagged` (`wire/decode.typ`) has one
  tag arm, `tag == bytes-type.type-name` → `bytes-type.decode(value,
  format: format)` (`type-name` and a real `decode` function now that
  `bytes.typ` exists, rather than a bare `"bytes"` string literal and an
  inline `bytes(value.data)`), harmless for `Cbor` since its own
  `cbor.encode` never produces that tag — its `bytes` values are caught by
  `decode`'s primitive fallback before ever reaching tag dispatch. `data`
  is a plain array of byte integers, not base64 — confirmed empirically
  Typst has no built-in base64 codec to match a base64 string against, so
  the array form needs no new dependency on either side. On the Rust side
  (a separate mechanism — see *Format threading* below for why the two
  sides diverge here), `Format::Cbor` and `Bytes`'s
  `Serialize`/`Deserialize` (`serialize_bytes`/`deserialize_byte_buf`) are
  untouched; `Json`/`Yaml`/`Toml::encode` instead build a `cbor4ii::core::
  Value` tree first (`value_from_owned`, a sibling to the existing
  `value_to_owned` `any.rs` already relies on), rewrite `Value::Bytes`
  nodes into the same tagged-array shape with `rewrite_bytes_to_array`,
  then serialize *that* tree; `decode` mirrors it with
  `rewrite_array_to_bytes` before `value_to_owned`. Both `value_from_owned`/
  `value_to_owned` go straight from `T`/`Value` to `Value`/`T` — a
  hand-written `Serializer`/`Deserializer` pair over `Value` directly
  (`format/value.rs`), not (an earlier version of this) a round trip
  through real CBOR bytes (`cbor4ii::serde::to_vec` then `from_slice`,
  chosen originally only because `cbor4ii`'s public `serde` module exposes
  no direct T-to-Value/Value-to-T conversion — confirmed by reading its
  source, `cbor4ii-1.2.3/src/serde.rs` — so building the tree without a
  custom (de)serializer meant paying for a real CBOR encode/decode pass
  just to get one). The custom pair needed one orphan-rule workaround
  (`struct ValueDeserializer(Value)`, since neither `Deserializer` nor
  `Value` are local to this crate) and was verified to preserve every wire
  edge case the byte round trip used to get for free, especially `()`
  encoding as an empty array rather than `null` (matching `cbor4ii`'s own
  `serialize_unit`) — checked with a byte-for-byte before/after diff across
  unit, `Option`, `Bytes`, `Vec<u8>`, negative/max `i64`, a struct, `Any`,
  and a native type, not assumed. This tree round-trip (still one, just no
  longer through real bytes) is the one real, non-mechanical piece of new
  logic anywhere in the type system, and it's confined to the three
  non-default formats — `Cbor` stays a single direct `cbor4ii::serde` pass,
  exactly as fast as before this feature existed. Measured impact (`tests/
  stress/warmup/`, per-call gap between `--timings`' `call plugin` spans —
  see *Performance* in `README.md` for the methodology): JSON ~924μs →
  ~717μs (−22%), TOML ~2,034μs → ~1,904μs (−6%), YAML unchanged
  (~7,500μs either way) — YAML's own `serde-saphyr` pass so dominates its
  total cost that removing one CBOR-byte pass doesn't register above
  run-to-run noise, even though `Yaml::encode`/`decode` go through the same
  `value_from_owned`/`value_to_owned` pair as `Json`/`Toml`.

A third problem surfaced only once the test suite below started exercising
every native type through every format: **`Yaml::decode` mis-resolved a
bare `y`/`n` map key as a boolean** (`Alignment`/`Center` both use literal
`x`/`y` field names, and both failed — `foundations/dictionary/struct`,
`layout/alignment`, `visualize/center`, `visualize/gradient`,
`visualize/paint` all round-trip a value with a `y` key somewhere). Root
cause, confirmed with a standalone probe: `serde-saphyr` follows legacy
YAML 1.1 core-schema scalar resolution (`y`/`n`/`yes`/`no`/`on`/`off` →
boolean, the "Norway problem") for any position it's asked to deserialize
*generically* — and `cbor4ii::core::Value`'s `Deserialize`, unlike
`serde_json::Value`'s, reads map keys generically (`deserialize_any`, not a
`String`-typed hint), since CBOR itself allows non-string map keys and the
type has to stay general enough to represent that. `{y: 1}` parsed straight
into `cbor4ii::core::Value` comes back `Map([(Bool(true), Integer(1))])`;
the *identical* bytes into `serde_json::Value` correctly come back `Object
{"y": 1}`, because `serde_json::Value` always deserializes keys through a
`String`-typed hint, sidestepping the ambiguous-scalar path entirely. Fix,
confined to `Yaml::decode` (`format/yaml.rs`) — `encode` is unaffected,
since the ambiguity is a parsing-time concern only: parse into
`serde_json::Value` first, then `serde_json::from_value` into
`cbor4ii::core::Value` (a second, purely in-memory conversion with no text
left to misinterpret), *then* run the normal `rewrite_array_to_bytes`/
`value_to_owned` pipeline. `yaml`'s Cargo feature pulls in `serde_json` as
a dependency unconditionally now (even with `json` off) for exactly this.

TOML has two more gaps of its own, both closed rather than left as
documented limitations: its grammar requires a top-level table (confirmed
`toml.encode(5)`/`toml::to_string(&42_i64)` both reject a bare scalar the
same way), and it has no `none`/null at all (confirmed `toml.encode((v:
none))` silently *drops* the key, and `toml::to_string` on a bare `None`
hard-errors with "unsupported None value" — two different failure modes on
the two sides for the same underlying gap). Both are handled unconditionally,
not just for the values that would otherwise fail, keeping `Toml`'s
encode/decode branch-free on "does this particular value need special
handling":
- **Top-level table.** Every encode wraps the whole tree under one fixed
  key (`"value"`) before it ever reaches `toml.encode`/`toml::to_string`;
  decode unwraps the same key first. Typst: `format/toml.typ::encode` is
  `toml.encode((value: wire.encode(value, format: "toml")))`, `decode` is
  `wire.decode(toml(data).value, format: "toml")`. Rust: `Toml::encode` builds
  `Value::Map(vec![(Value::Text("value"), tree)])` before serializing;
  `decode` finds and extracts that key from the parsed envelope, erroring
  clearly if it's missing or the parse wasn't even a table.
- **`none`/null.** Rewritten into the tagged dictionary `{typwire-type:
  "none"}`, the same pattern as `Bytes` above, and — also the same as
  `Bytes` — decode is fully shared: `decode-tagged` (`wire/decode.typ`) has
  a `"none"` arm that just returns `none`, harmless for the three formats
  that never produce that tag. Encode is format-specific on both sides, but
  the two sides differ in *where*: on the Rust side,
  `rewrite_none_to_tagged` (`format/toml.rs`) is still its own separate
  tree-walk over an already-built `Value`, the same shape
  `rewrite_bytes_to_array`/`rewrite_array_to_bytes` use (see *Format
  threading* below for why Rust keeps this as a separate pass while Typst
  doesn't); on the Typst side, it's folded into the same single
  `wire/encode.typ::encode` pass as `bytes` — a `content == none` branch,
  reached only when `format == "toml"` (every other format's `none` is
  already primitive), builds the tagged dictionary directly. The
  once-separate `wire/rewrite-none.typ::none-to-tagged` this replaced has
  been deleted.

Between the envelope and the `bytes`/`none` handling above, there is no
longer any value shape `typwire.toml`/`Toml` can't round-trip.

#### Format threading (Typst side)

`wire/encode.typ::encode(content, format: "cbor")` and
`wire/decode.typ::decode`/`decode-tagged` take `format` as a **named**
argument with a default of `"cbor"`, and every single `wire/types/**`
module's `encode`/`decode` does too — including the 31 of them (`path`,
`symbol`, `length`, every color space, every gradient kind, ...) that
never branch on it and just ignore it. `format` started out positional and
required (an earlier version of this threading had it that way), then was
changed to named with a default of `"cbor"` everywhere, not just at the
public entrypoints in `src/lib.typ` — matching `src/lib.typ`'s own
generic `encode(value, format: "cbor")` shape all the way down, and
meaning a bare `xxx.encode(value)` (no `format:` at all) is valid
everywhere in this layer and means exactly `"cbor"`, the same as what
every one of these functions did before `format` existed at all. This is
deliberately more uniform than strictly necessary, since only two things
in the whole tree walk are actually format-sensitive (`bytes`, its own
dedicated `wire/types/foundations/bytes.typ` module now, and — on TOML
specifically — `none`, still handled directly in `encode.typ::encode`, see
above), but every `encode`/`decode` pair takes `format` anyway, so a
future type that *does* need to branch on it never requires a signature
change, and every function in this layer reads the same way at a glance.
The one thing this threading replaced: `wire/rewrite-bytes.typ`/
`rewrite-none.typ`, two
now-deleted files that used to walk the *already-built* tree a second
(and, for TOML, third) time after `encode.typ::encode` finished, rewriting
`bytes`/`none` leaves in place. Folding that into the single recursive
`encode` pass means JSON/YAML save a full extra tree-walk and TOML saves
two, at the cost of `format` appearing in every signature in this layer.

`is-primitive` (`wire/encode.typ`) is the function this generalizes most
directly: it used to be an unparameterized "is this one of the primitive
Typst types CBOR can carry as-is" check (with `bytes` unconditionally
`true`, since that was only ever called from the CBOR-only encoder this
crate started with). It's now `is-primitive(content, format: "cbor")` —
`int`/`float`/`str`/`bool` are primitive in every format as before, and
`none` is primitive in every format *except* `"toml"`; `bytes` isn't part
of this check at all anymore (see the *Bytes* bullet above for where its
format-sensitivity lives instead, now that it has its own module). Content
`is-primitive` says `false` for falls through to `encode`'s `bytes`/`none`
handling instead of passing through untouched.

The Rust side does **not** get the equivalent treatment, and deliberately
so: it has no per-native-type file that would need a `format` parameter
threaded through in the first place (`Length`, `Path`, etc. each derive
their wire shape from a single shadow struct via Serde, format-agnostic by
construction), and `Format::Json`/`Yaml`/`Toml::encode` already dispatch
per-format at compile time through the generic `T: Typwire` + marker-type
mechanism described above — there's no analogous "one recursive function
walks every value" chokepoint to thread a runtime `format` string through.
Its `rewrite_bytes_to_array`/`rewrite_none_to_tagged` therefore remain
separate tree-walks over a `cbor4ii::core::Value`, exactly as described in
the *Bytes*/TOML sections above.

### Native Typst value representation

Typst types with no CBOR primitive (`Length`, `Angle`, `Ratio`, `Alignment`,
`Color` and its 8 spaces, `Gradient` variants, `Stroke`, `DateTime`,
`Duration`, `Version`, `Type`, `Center`, `Stop`, `Auto`, `Path`, `Symbol`,
`Regex`, `Decimal`, `Arguments`, `Label`) are represented as **tagged
dictionaries**: an ordinary map carrying a `typwire-type` string field
(kebab-case, e.g. `"length"`, `"color-oklch"`, `"gradient-linear"`,
`"alignment"`, `"stroke"`, `"auto"`, `"path"`, `"symbol"`, `"regex"`,
`"decimal"`, `"arguments"`, `"label"`). `Radius` is the one exception: it has
no matching Typst runtime type at all (`rect(radius: ...)` just takes a plain
dictionary of corner keys), so it round-trips as an ordinary untagged dict —
see the `DashPattern` paragraph below for another type in the same
situation.

Typst types that *do* have a CBOR primitive (`int`, `float`, `str`, `bool`,
`array`, `dictionary`) need none of the above: `i64`/`f64`/`String`/`bool`/
`Vec<T>`/`BTreeMap<String, T>` are already `Typwire` directly (`wire.rs`),
and Typst's own `cbor.encode`/`json.encode`/etc. already produce exactly
that shape for them. `Int`/`Float`/`Str`/`Bool`/`Array<T>`/`Dictionary<T>`
(`foundations/{int,float,str,bool,array,dictionary}.rs`) are purely additive
named wrappers around those same primitives, for a plugin author who wants
an exported signature to read `Int`/`Array<T>`/etc. rather than `i64`/
`Vec<T>`/etc. — matching Typst's own reference type names the way `Length`/
`Angle`/etc. already do for the tagged types. Each is `#[serde(transparent)]`
over a single private field (no `…Typst` shadow struct, no tag, no `TryFrom`
validation — there's nothing to validate: the wire shape *is* the
primitive's), so `encode(&Int::new(5))` and `encode(&5_i64)` produce
byte-identical output on every format, confirmed per-type in each file's own
test (`wire_shape_is_identical_to_a_bare_i64` etc.) rather than assumed.
Because the two spellings are wire-identical, the Typst side needs zero
changes to support them — this is the one category of type in this section
that requires no matching `src/wire/types/**` file at all. `Array<T>`/
`Dictionary<T>` get their `impl<T: Typwire> Typwire for ...` written directly
alongside the type (same pattern `OneOf2`..`OneOf8` use in `one_of.rs`)
rather than through the crate-root `impl_typwire!` macro, since that macro
only takes concrete (non-generic) types — `Int`/`Float`/`Str`/`Bool` do go
through it, same as every other concrete native type.

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
  Each `<type>.rs` defines a private `…Typst` shadow struct with
  `#[serde(try_from = "…Typst", into = "…Typst")]` on the public type
  (named for the Typst-side wire shape it mirrors, not for any one format —
  every format's encode/decode passes through it); `const TYPE_NAME` is the
  tag, `TryFrom` verifies it.
- Typst: `src/wire/types/{foundations,layout,visualize}/<type>.typ` each
  expose `type-name`, `encode`, `decode` (`Center` and `Stop` live under
  `src/wire/types/visualize/gradient/`, since they're only ever nested inside
  a gradient, never a standalone Typst value). `src/wire/encode.typ` and
  `decode.typ` themselves stay at `src/wire/`, outside that category split,
  since they aren't type-specific — see *Wire formats* above for why this
  directory is named `wire/`, not `cbor/`. `encode.typ::encode` dispatches on
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
via `utils.typ::unescape-repr-string`/`unescape-repr-content` (backslash/
quote/`\n`/`\t`/`\r`/`\u{...}`, confirmed against Typst's actual escape set
empirically since it isn't fully spelled out in the docs) — deliberately
*not* `eval`, since that would execute the string as Typst code; this is
pure string manipulation instead. `symbol.typ`/`regex.typ` need the same
trick and share it (`unescape-repr-string` is a thin `.slice(1, -1)` wrapper
around `unescape-repr-content`).

`unescape-repr-content` has to apply all of `repr`'s escapes in **one**
regex pass (`escape-pattern`, one alternation, one `.replace()` with a
dispatching callback), not several independent `.replace()` calls run one
after another. With independent passes, an escaped backslash immediately
followed by a literal `n`/`t`/`r`/`"` — e.g. the 4-character string
`\d\n` (backslash, d, backslash, n), `repr`'d as `\\d\\n` — gets corrupted:
a later pass's search for `\n` matches across the boundary an earlier pass
already consumed, turning the trailing literal `n` into an actual newline.
This was a real bug caught during `regex` support (paths can never contain a
backslash, so it never surfaced there; regex patterns are full of them) —
reproduced with the naive multi-pass version, then verified fixed against it
empirically before landing.

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

`regex` behaves like `path` for encoding purposes: no `str()` conversion, so
`regex.typ::encode` recovers the pattern from `repr()`
(`repr(value).slice(6, -1)`) via the same `unescape-repr-string` helper.
Unlike `path`, a compiled pattern reliably `==`s another compiled from the
same string (confirmed empirically), so `regex` round-trips `==`-exact with
no `symbol`-style caveats.

On the Rust side, `Regex` is feature-gated entirely (`foundations/regex.rs`,
crate feature `regex`, backed by `regex-lite`, optional dependency) —
`#![cfg(feature = "regex")]` as the file's first line, so the whole module
(the type, `Any::Regex`, and every `Typwire`/`From` impl involving it)
simply doesn't exist without the feature, confirmed by compiling a probe
that names `typwire::Regex` and getting a real "not found" error pointing at
the gate, not a runtime failure. There's deliberately no placeholder type
for the disabled case (an earlier version had one, always erroring on
decode) — a compile error for code that references `Regex` without the
feature is a stronger, earlier signal than a value that builds fine and only
fails when actually decoded. `Any`'s `"regex"` match arm in `from_map` (and
the `Any::Regex` enum variant and its `Serialize` arm) are individually
`#[cfg(feature = "regex")]`-gated too, so a `regex`-tagged value reaching
`Any` without the feature falls through to `Any`'s ordinary "unknown
typwire-type tag" error instead of a dedicated message — an acceptable
loss of specificity for not having to maintain a whole parallel
always-errors implementation.

With the feature, `Regex` follows the same `…Typst` shadow-struct convention
as every other native type (`#[serde(try_from = "RegexTypst", into =
"RegexTypst")]`) even though its real field is a foreign `regex_lite::Regex`
that isn't itself `Serialize`/`Deserialize` — `try_from`/`into` only need
*`RegexTypst`* to implement those, not `Regex`, so this works the same way it
does for every other type here. `TryFrom<RegexTypst>` compiles the pattern
immediately via `regex_lite::Regex::new`, so a malformed pattern fails at
decode time rather than producing a value that silently doesn't work as
advertised, and `Regex` derefs to `regex_lite::Regex` so `is_match`/`find`/
etc. are directly usable. `typwire-tests-plugin` builds with
`features = ["decimal", "regex"]` so the Tytanic suite exercises the real
thing for both.

`decimal` is the simplest of the four "extra" foundations types: unlike
`path`/`symbol`/`regex`, it has a real `str()` conversion that gives back
its exact digits directly, so `decimal.typ::encode` needs no `repr()`/
unescaping trick at all. The one real gotcha, caught empirically rather than
assumed: Typst's `Display` for a *negative* decimal writes the typographic
minus sign U+2212, not ASCII `-` (confirmed by reading
`typst-library/src/foundations/decimal.rs` upstream — its `Display` impl
writes `repr::MINUS_SIGN` explicitly before the magnitude). Typst's own
`decimal(...)` constructor happens to accept that character back (it
normalizes internally), but the Rust side parses the wire string directly
with `rust_decimal::Decimal::from_str_exact`, which only understands ASCII
`-` — so `decimal.typ::encode` normalizes U+2212 to `-` before it ever
reaches the wire, rather than relying on Rust-side preprocessing to undo a
Typst-specific typographic choice. `tests/foundations/decimal/test.typ`
covers negative values specifically as a regression test for this.

`Decimal` (`foundations/decimal.rs`) is feature-gated exactly like `Regex` —
crate feature `decimal`, `#![cfg(feature = "decimal")]` gating the whole
module, no placeholder type, `Any::Decimal` individually gated in `any.rs`
the same way `Any::Regex` is — and follows the same `…Typst` shadow-struct
convention. It's backed by `rust_decimal::Decimal`, which is worth calling
out: that's not just *a* decimal crate, it's the *same* one Typst's own
`decimal` type wraps internally (`pub struct
Decimal(rust_decimal::Decimal);` in Typst's own source), so this is Typst's
actual representation, not a reimplementation of it. `TryFrom<DecimalTypst>`
uses `rust_decimal::Decimal::from_str_exact`, the same exact (non-rounding)
parse Typst's own `FromStr` impl for `decimal` uses, and `Decimal` derefs
to `rust_decimal::Decimal` so its arithmetic methods are directly usable.

`arguments` needs no `repr()` trick either: it has a real constructor
(`arguments(..pos, ..named)`) and real accessors (`.pos()` → array, `.named()`
→ dictionary), so `arguments.typ::encode` just calls those directly and
`decode` spreads them back through the constructor. Confirmed empirically
that order between the positional and named groups doesn't need preserving
across the split (only within each group) — `arguments(..a.pos(), ..a.named())`
is `==` to a real captured `a` with interleaved positional/named arguments,
and the empty case (`arguments()`) round-trips `==` too. Unlike `Regex`/
`Decimal`, `Arguments` (`foundations/arguments.rs`) needs **no feature gate**:
it wraps no external crate, only `Vec<Any>`/`BTreeMap<String, Any>` — already
unconditionally available — so it's wired into `any.rs` the same way the
always-on native types are (a plain `native_conversions!` entry, not a
standalone `#[cfg(...)]`-gated `From` impl). It still follows the same
`…Typst` shadow-struct convention as everything else. Because `Any` doesn't
derive `PartialEq`/`Eq`, neither does `Arguments`; the Tytanic suite's
`roundtrip` helper compares via Typst's own `==`/`repr()` on the decoded
value instead, so this doesn't affect round-trip verification. This is also
the first native type whose Rust representation directly embeds `Any`
(mutual recursion between the two types is unproblematic — ordinary
type-level recursion within one crate, not a cyclic dependency).

`label` is the simplest type added so far: like `decimal`, it has a real
`str()` conversion (documented as the supported way to read a label's name),
so `label.typ::encode`/`decode` need no `repr()` handling at all —
`label(str(value))` round-trips `==`-exact for every case tried, including
names the dedicated `<...>` syntax can't express (spaces, non-ASCII
characters), since the constructor "accepts any non-empty string" per
upstream's own doc comment.

`version` accepts any number of components at construction (`version()`,
`version(1)`, `version(1, 2, 3, 4, 5, 6, 7)`, ... are all valid Typst — there
is no fixed major/minor/patch/revision/build scheme, confirmed empirically
against an earlier version of this module that assumed exactly five and
silently zero-padded anything shorter). `.at(n)` can't reveal how many
components a value actually has: it returns `0` for any `n` past the real
end rather than erroring (confirmed: `version(1, 2).at(4)` is `0`), and
equality is lenient to that same padding (`version(1, 2) ==
version(1, 2, 0, 0, 0)` is `true`), so neither can distinguish "has 2
components" from "has 5, with 3 trailing zeros." `repr()` is the one place
that does: `repr(version(1, 2))` is the literal source `"version(1, 2)"`, so
`version.typ::component-count` strips the `"version("`/`")"` wrapper and
counts `", "`-separated entries in what's left (`""` specifically means
zero, not one empty entry) — text parsing used *only* to get a count, not
to read the actual values back, per the same "don't trust `repr()` for data,
only shape" caution `path`/`symbol`/`regex` already follow elsewhere in this
file; `.at(i)` for `i` in that range then reads the real `i64` components
directly. `encode`'s debug assertion needed the same `std.version`
qualification `type.typ` already uses for `type` — `version`'s own
parameter is named `version`, shadowing the builtin the same way, and the
unqualified form (`type(version) == version`) is a type-vs-value comparison
that's *always* false; confirmed as a real, previously-undetected bug this
way (silently never caught before since the roundtrip suite never enables
`--input debug-assert=true` — see *Commands* above). On the Rust side,
`Version` (`foundations/version.rs`) is `{ components: Vec<i64> }` rather
than five named fields, following the same `…Typst` shadow-struct
convention as every other tagged type.

`alignment` has no `str()` conversion and no accessor that returns a bare
string either — `.x`/`.y` give back sub-`alignment` values (or `none`), and
`repr()` of one is unquoted Typst source (`left`, `top`, `left + top`), not
a string literal. Rather than parse that text like `path`/`symbol`/`regex`
do, `alignment.typ` matches each component against Typst's own global
constants by equality (`value == left`, `value == top`, ...) — there are
only 5 horizontal and 3 vertical values, all fixed, so an equality chain is
simpler and more robust than a text-parsing trick, and avoids depending on
`repr()`'s formatting at all. On the Rust side, `Alignment` (`layout/
alignment.rs`) mirrors upstream's own `Alignment::H`/`V`/`Both` shape as
`{ x: Option<HAlignment>, y: Option<VAlignment> }`, with `TryFrom` rejecting
the invalid all-`None` case at decode time. `HAlignment`/`VAlignment` are
plain nested enums (serialized as their lowercase name, e.g. `"left"`) —
like `ColorSpace`, they carry no `typwire-type` tag of their own and so have
no `Any` variant, only `impl_typwire!` (for standalone export use).

`stroke` turned out to need no `repr()` trick or even a hand-rolled
encode/decode shape at all: every field, once read off a real `stroke`
value, is *already* one of the wire's other supported shapes — `paint` is a
`color`/`gradient` (or `auto`), `thickness` a `length` (or `auto`), `cap`/
`join` plain strings (`"round"`, `"bevel"`, ...; Typst normalizes its
enum-like constructor arguments to strings by the time they're readable back
off the object), and `dash` is `none`, `auto`, or a plain dictionary
`(array: (...), phase: ...)` whose `array` entries are lengths or the literal
string `"dot"` — confirmed empirically field-by-field, including that
`stroke(paint: .., thickness: .., cap: .., join: .., dash: .., miter-limit:
..)` reconstructed from a stroke's own fields is `==` to the original.
`stroke.typ::encode` therefore just reads the six fields into a dict and
`decode` spreads them straight back into the constructor, relying entirely
on `encode-inner`/`decode-inner`'s existing recursive handling — the same
"just call the accessors" shape `arguments.typ` uses, taken one step
further since there's no dedicated per-field logic left to write at all.
Pattern/tiling paints have no `Typwire` representation (no such type exists
in this crate yet) and fail the same way any other unsupported value does.
On the Rust side, `Stroke` (`visualize/stroke.rs`) wires every field through
`OneOf2<Auto, _>` rather than introducing a dedicated "smart value" wrapper
type — `OneOf2`/`OneOf3` already exist for exactly this "value or other
shape" composition (see the README's own example), so a Typst `Smart<T>`
field just becomes `OneOf2<Auto, T>` here, no new abstraction needed.
`Paint` is a plain type alias for `OneOf2<Color, Gradient>` (already
`Typwire` via `OneOf2`'s blanket impl, so it's *not* separately listed in
`impl_typwire!` — doing so would be a duplicate impl of the same concrete
type under two names and fail to compile). `LineCap`/`LineJoin` are plain
nested enums like `HAlignment`/`VAlignment` above. `DashPattern`'s wire
shape is a plain dictionary, not a tagged one — the same situation `Radius`
is already in (see the type list above) — because Typst represents a dash
pattern as an ordinary dictionary too, not a distinct runtime type; its
`array` entries (`DashLength`) need one small hand-written `Serialize`/
`Deserialize` impl instead of the usual derive, since `"dot"` is a plain
string sibling to `Length`'s own tagged-dict shape, not something an
`#[serde(untagged)]` derive over `Length` alone could express.

Adding or renaming a native type means editing both sides plus the tag lists in
`any.rs` (`from_map`) and `src/wire/decode.typ`.

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

`#[typwire::export(rename = "wire-name")]`/`#[typwire::export(format =
"json")]` parse two independent options (`syn::meta::parser` in the
`export` entry point, combinable) — see *Wire formats* above for `format`.
Since `#[wasm_func]` always exports under the
wrapper's *identifier*, the rename path skips `#[wasm_func]` entirely: it emits
an `extern "C"` fn with `#[unsafe(export_name = "wire-name")]` that re-declares
the two `typst_env` protocol imports, reads/splits the argument buffer itself,
and calls the same decode/encode body. So `rename` accepts arbitrary strings
(kebab-case included); the no-rename path is byte-for-byte unchanged. A
kebab-case `wire-name` is reachable directly as `plugin.wire-name` (see
*Testing* above); only a `rename` string that isn't identifier-shaped at
all (a space, a leading digit, ...) genuinely needs
`dictionary(plugin).at("wire-name")` instead.

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

`src/lib.typ` imports each format module (`format/{cbor,json,yaml,toml,
debug}.typ`) under a private `-format` suffix (`cbor-format`, ...), builds
a `formats` dictionary mapping each name to its module, and defines the
public API on top of that in two layers:

- **Generic, `format:`-argument functions** — `encode(value, format:
  "cbor")`, `decode(data, format: "cbor")`, `call(function, ..args, format:
  "cbor")` — each just `resolve-format(format).<fn>(...)`, where
  `resolve-format` is `formats.at(format, default: none)` plus a clear
  panic (naming the bad string and the four valid ones) instead of Typst's
  generic "dictionary does not contain key" error on a typo.
- **Per-format namespaces** — `#let cbor = cbor-format` (and likewise
  `json`/`yaml`/`toml`) — each namespace is the real module re-exported
  directly under its public name, **not** wrapped in a plain dictionary of
  closures. That distinction matters and was confirmed the hard way: Typst
  allows `module.field(..)` dot-call sugar (`typwire.cbor.call(f, x)`) but
  refuses the identical syntax when the receiver is a plain dictionary
  whose value happens to be a function (`"cannot directly call dictionary
  keys as functions"`) — an earlier version of this layer built `cbor`/
  `json`/`yaml`/`toml` as `(encode: ..., decode: ..., call: ...)` dicts
  wrapping the generic functions, which compiled fine but broke every
  `typwire.<format>.call(...)` call site across the whole Tytanic suite the
  moment it ran. Re-exporting the modules directly sidesteps this
  entirely, and — since `formats` points at those same modules — the
  generic functions and the namespaces run the exact same code either way;
  `typwire.<format>.encode(value)` and `typwire.encode(value, format:
  "<format>")` are interchangeable, never two implementations to keep in
  sync.

`cbor`/`json`/`yaml`/`toml` each have the same `encode`/`decode`/`call`
shape: `call(function, ..args)` rejects named args, encodes each positional
arg **independently**, calls the plugin function with the resulting byte
strings (wrapped in `bytes(..)` first for the three formats whose own
`.encode` returns `str`, not `bytes` — confirmed empirically that a Typst
plugin call rejects a bare `str` argument outright), and decodes the single
returned byte string. This positional, separately-encoded calling
convention is what `#[typwire::export]`'s `&[u8]`-per-parameter wrapper
expects, for every format. Each format's own `encode`/`decode` is a thin
wrapper around `wire.encode(value, format: "<format>")`/`wire.decode(data,
format: "<format>")` — see *Format threading* above for what that argument
actually does.

`src/format/debug.typ` is the odd one out: `debug.encode(value, format:
"cbor")`/`debug.decode(data, format: "cbor")` are `wire.encode`/
`wire.decode` themselves, exposed directly with no serialization step and
therefore no `call` (there is no `bytes`/`str` to hand a plugin) — useful
for inspecting or testing the tree Typwire produces without going through
an actual format or a plugin round trip. Its own `format:` argument isn't
about serialization (there is none here) — it only picks which format's
`bytes`/`none` tagging rules the returned tree previews, since that's the
one part of the tree that's still format-sensitive after *Format
threading* above. Default `"cbor"` means `debug.encode`'s default output
keeps `bytes`/`none` fully native, matching how the tree looked before
per-format tagging existed at all.

## Explicitly unsupported (by design)

Borrowed values (`&str`, `&[u8]`, borrowed models), Typst `content`,
`function`, fractions, directions, and relative lengths. `content`/`function`
encoding panics on the Typst side; references are a compile error via the macro.
