//! Wasm plugin for the Typst roundtrip test suite.
//!
//! Every test in `<repo>/tests/<name>/` (`<name>` may nest, e.g.
//! `foundations/bool`) ships **four** sibling cases, one per wire format --
//! `tests/<name>/{cbor,json,yaml,toml}/`, each its own `plugin.rs` next to
//! its `test.typ`. `plugin.rs` usually has one exported function
//!
//! ```ignore
//! #[typwire::export(rename = "roundtrip-<name>-<format>")]
//! fn roundtrip_<name>_<format>(value: T) -> Result<T, String> { Ok(value) }
//! ```
//!
//! (`format`/`rename` omitted for the `cbor` case, since `cbor` is
//! `#[typwire::export]`'s own default -- though `rename` is still given, for
//! a uniform `dictionary(plugin).at("roundtrip-<name>-<format>")` access
//! pattern across all four), though a case is free to define several
//! functions when they're closely related (see `tests/rename/*/plugin.rs`).
//! Every group of four is pulled in here with one `case!` call.
//! `#[typwire::export]` expands to a `#[wasm_func]` wrapper that needs the
//! `initiate_protocol!()` items in scope, which only holds at the crate
//! root, so the case files are `include!`d here rather than mounted as
//! `mod`s.
//!
//! Rules for a `plugin.rs` case file, because its tokens land in this module:
//! - no `use` items (they would collide across cases); rely on the glob imports
//!   below or write fully-qualified paths,
//! - every exported function name (and any locally defined type, e.g. a
//!   `#[derive(Typwire)]` struct) must be unique across the whole suite --
//!   hence the `roundtrip_<name>_<format>` convention and, where a case
//!   needs a local struct, a `<Name><Format>` naming pattern (see
//!   `tests/foundations/dictionary/struct/*/plugin.rs`).
//!
//! To add a test: `mkdir -p tests/<name>/{cbor,json,yaml,toml}`, add a
//! `test.typ` + `plugin.rs` pair to each, then add one `case!("<name>");`
//! line below -- it expands to all four `include!`s.

#![allow(unused_imports)]

typwire::initiate_protocol!();

use serde::{Deserialize, Serialize};
use typwire::*;

/// Pulls in all four format variants of one test: `tests/<name>/{cbor,json,
/// yaml,toml}/plugin.rs`.
macro_rules! case {
    ($name:literal) => {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../../tests/",
            $name,
            "/cbor/plugin.rs"
        ));
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../../tests/",
            $name,
            "/json/plugin.rs"
        ));
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../../tests/",
            $name,
            "/yaml/plugin.rs"
        ));
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../../tests/",
            $name,
            "/toml/plugin.rs"
        ));
    };
}

// No matching page under https://typst.app/docs/reference/: `Any` and
// `OneOfN` are typwire-specific dynamic-dispatch mechanisms with no single
// corresponding Typst type; `rename` exercises the `#[typwire::export]`
// macro option, not a value type; `Box<T>` is wire-transparent (same shape
// as `T`); `unit`/`char` have no Typst type of their own either.
case!("any");
case!("one-of");
case!("rename");
case!("box");
case!("unit");
case!("char");

// The rest mirror https://typst.app/docs/reference/ category by category:
// one dir per Typst type name, not per Rust type -- e.g. this is `int`, not
// `i64`. When more than one Rust type can receive the same Typst type, that
// dir holds subtests (one per Rust type) instead of a `plugin.rs`/`test.typ`
// pair directly, as `int`, `bool`, `float`, `str`, `array`, and `dictionary`
// do below -- each of those six now also has a `wrapper` subtest for its
// named `Int`/`Bool`/`Float`/`Str`/`Array<T>`/`Dictionary<T>` type
// (`typwire::Int`'s doc comment explains why that type exists).

// https://typst.app/docs/reference/foundations/
case!("foundations/arguments");
case!("foundations/auto");
case!("foundations/bytes");
case!("foundations/datetime");
case!("foundations/decimal");
case!("foundations/duration");
case!("foundations/label");
case!("foundations/none");
case!("foundations/path");
case!("foundations/regex");
case!("foundations/symbol");
case!("foundations/type");
case!("foundations/version");

// `bool`/`float`/`str` each have two Rust representations -- the bare
// primitive and the named `Bool`/`Float`/`Str` wrapper (`typwire::Int`'s
// doc comment explains why the wrapper exists) -- so, like `int`/`array`/
// `dictionary` below, they're subtest dirs rather than a single
// `plugin.rs`/`test.typ` pair.
case!("foundations/bool/bool");
case!("foundations/bool/wrapper");
case!("foundations/float/float");
case!("foundations/float/wrapper");
case!("foundations/str/str");
case!("foundations/str/wrapper");

case!("foundations/int/i64");
case!("foundations/int/u8");
case!("foundations/int/wrapper");

case!("foundations/array/vec");
case!("foundations/array/vec-deque");
case!("foundations/array/linked-list");
case!("foundations/array/btree-set");
case!("foundations/array/hash-set");
case!("foundations/array/fixed-size");
case!("foundations/array/tuple");
case!("foundations/array/wrapper");

case!("foundations/dictionary/btree-map");
case!("foundations/dictionary/hash-map");
case!("foundations/dictionary/struct");
case!("foundations/dictionary/wrapper");

// https://typst.app/docs/reference/layout/
case!("layout/alignment");
case!("layout/angle");
case!("layout/length");
case!("layout/ratio");
case!("layout/radius");

// https://typst.app/docs/reference/visualize/
case!("visualize/color");
case!("visualize/gradient");
case!("visualize/stroke");

// No dedicated reference page of their own, but grouped here since they only
// exist in service of the types above: `color-space` is the gradient/color
// `space` argument, `paint` is the color-or-gradient union used for fills
// and strokes, `center`/`stop` are gradient-only helper shapes.
case!("visualize/color-space");
case!("visualize/paint");
case!("visualize/center");
case!("visualize/stop");
