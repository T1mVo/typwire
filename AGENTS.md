# AGENTS.md

## What this is

`typwire` is a library for typed, Serde-backed data exchange across the Typst ⇆ WASM-plugin boundary. Two packages that must stay **version-locked** (`0.2.0`, no compile-time guard — a mismatch fails at the wire level):

- **Typst package** (`src/`): `lib.typ` (public API), `typwire.typ`, `format/{cbor,json,debug}.typ`, `wire/` (encode/decode tree layer + `wire/types/**` per-type modules).
- **Rust crates** (`src/core/crates/`): `typwire` (runtime), `typwire-macros` (`#[typwire::export]`, `#[derive(Typwire)]`; reuses `typwire`'s README), `typwire-tests-plugin` (wasm-only test cdylib).

Bumping the version: `typst.toml`, `src/core/Cargo.toml` (`workspace.dependencies`), and every crate `Cargo.toml`, plus `@preview/typwire:x.y.z` snippets in `README.md` and `crates/typwire/README.md`.

Two wire formats: **CBOR** (default, fast path) and **JSON** (crate feature `json`). Optional features `decimal`, `regex`. The `Cbor`/`Json` marker types implement `Format` (`src/format/*.rs`); the Typst side threads a named `format:` argument (default `"cbor"`) through every `wire/**` encode/decode.

## Commands

```sh
cd src/core
cargo test                                          # typwire + typwire-macros, unit + doctests
cargo test -p typwire --features decimal,regex,json # all features
cargo build --target wasm32-unknown-unknown         # plugin-target build
```

Scripts are Nushell (`scripts/*.nu`, run from the repo root):

```sh
nu -c 'use scripts; scripts test'            # release-build test plugin, wasm-opt, then `tt run` (args forwarded to tt)
nu -c 'use scripts; scripts package'         # assemble Typst package into target/typwire/<version>/
nu -c 'use scripts; scripts install'         # package + copy into the local Typst package dir
nu -c 'use scripts; scripts resources'       # recompile resources/*.typ to SVG
nu benchmarks/run.nu [--runs N] [workload…]  # typwire vs sertyp, needs hyperfine
```

Needs `tt` ([Tytanic](https://typst-community.github.io/tytanic/)) and `wasm-opt` (Binaryen) on `PATH`. `src/core/.cargo/config.toml` enables `+bulk-memory,+simd128` for wasm32 (`wasm-opt` therefore needs `--enable-bulk-memory-opt --enable-simd`).

## Testing

The real suite is the Typst roundtrip suite in `tests/<type>/{cbor,json}/` (Tytanic `compile-only`), each with:

- `test.typ` — imports `../../common.typ`, asserts a Typst→Rust→Typst roundtrip via `typwire.<format>.call`. Exports are reached by kebab-case field access: `plugin.roundtrip-<type>-<format>` (a `rename` that isn't identifier-shaped needs `dictionary(plugin).at(...)`; only `tests/rename/` does).
- `plugin.rs` — `#[typwire::export(format = "<format>", rename = "roundtrip-<type>-<format>")] fn roundtrip_<type>_<format>(v: T) -> Result<T, String>`.

`plugin.rs` files are `include!`d into the crate root of `typwire-tests-plugin/src/lib.rs`, so: **no `use` items**, and every fn / locally defined type name must be **globally unique** (`PointCbor`/`PointJson`). Add a test with `mkdir -p tests/<name>/{cbor,json}`, a `test.typ` + `plugin.rs` per format, and one `case!("<name>");` line in `lib.rs`. The plugin is built to `tests/target/plugin.wasm` (git-ignored). `tests/stress/*` are deliberately heavy; Rust-only logic (e.g. `DateTime` timestamp helpers) is covered by unit tests in the crate. Typst debug assertions in `src/wire/**` need `--input debug-assert=true` and are off in the suite.

## Architecture

- **Wire contract**: `Typwire` (`wire.rs`) is a marker trait over `Serialize + DeserializeOwned` with explicit opt-in — deriving Serde alone is not enough. Top-level `encode`/`decode` always mean CBOR.
- **Native Typst types** (`Length`, `Color`, `Gradient`, `Stroke`, `DateTime`, `Regex`, …) travel as **tagged dictionaries** with a `typwire-type` string. The tag must be the **first key** (decoding dispatches on it without buffering). Primitives (`int`, `float`, `str`, `bool`, array, dictionary) are untagged; `Int`/`Float`/`Array<T>`/… are wire-identical named wrappers. `bytes` is native in CBOR; in JSON it becomes a tagged array (`wire/types/foundations/bytes.typ`, `format/json.rs`).
- **Rust layout** mirrors Typst's reference categories: `foundations/`, `layout/`, `visualize/` (`mod.rs` is re-exports only; each type has a private `…Typst` shadow struct using `#[serde(from/try_from/into)]` and a zero-sized `Tag<Self>`). `Any` and `OneOf2..8` (untagged, variants tried in order) live at the crate root. `Regex`/`Decimal` are feature-gated at the module level.
- **Typst layout** mirrors it under `src/wire/types/`; each module exposes `type-name`, `encode`, `decode`. Adding/renaming a native type means editing both sides plus `any.rs` (`native`) and `wire/decode.typ` (`decoders`).
- **`#[typwire::export]`** (`rename`, `format` options) wraps a free `fn(A, B) -> Result<T, E>` in a `#[wasm_func]`-style wrapper taking one `&[u8]` per argument. References in argument types, generics, `async`, etc. are rejected. `rename` bypasses `#[wasm_func]` and emits an `extern "C"` fn with `export_name`.
- **Typst entrypoint** `src/lib.typ`: generic `encode`/`decode`/`call(…, format:)` plus per-format namespaces `typwire.cbor`/`.json`/`.debug`, which must be the **modules** re-exported directly (a dict of closures breaks `typwire.cbor.call(..)` dot-call syntax). `call` encodes each positional arg separately.

## Performance rules

CBOR speed matters; JSON is for readability. Don't regress these:

- Typst: avoid closure calls on large args (memoization hashes them), iterate with `for` rather than `.map(closure)`, and return containers with no nested native values as-is.
- Tags first (see above); no nested tags where the enclosing tag fixes the type (color components and gradient stops are bare floats).
- wasmi multiplies Rust-side cost ~20×; saving an allocation per value is worth it.

## Unsupported by design

Borrowed values (`&str`, `&[u8]`), Typst `content`, `function`, fractions, directions, relative lengths. `content`/`function` encoding panics on the Typst side; references are a compile error in the macro.

The working copy uses Jujutsu (`.jj/`) on top of Git; plain `git` works.

## Commits

After every change, suggest a commit message (don't commit unless asked) matching the style of the existing history: a single lowercase `<scope>: <imperative summary>` line, no trailing period, e.g. `core: add DateTime::unix_timestamp`, `docs: revise README`, `perf: avoid redundant type lookups`, `chore: set version to 0.2.0`. Common scopes/types: `core`, `docs`, `test`, `perf`, `refactor`, `chore`, `build`, `scripts`, `benchmarks`, `feat`.
