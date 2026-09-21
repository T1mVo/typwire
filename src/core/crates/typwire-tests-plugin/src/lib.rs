//! Wasm plugin for the Typst roundtrip test suite.
//!
//! Every test in `<repo>/tests/<name>/` (`<name>` may nest, e.g.
//! `foundations/bool`) ships its own `plugin.rs` next to its `test.typ`.
//! Usually that's one exported function
//!
//! ```ignore
//! #[typwire::export]
//! fn roundtrip_<name>(value: T) -> Result<T, String> { Ok(value) }
//! ```
//!
//! per file, though a case is free to define several when they're closely
//! related (see `tests/rename/plugin.rs`).
//! Every case is pulled in here with `case!`. `#[typwire::export]` expands to
//! a `#[wasm_func]` wrapper that needs the `initiate_protocol!()` items in
//! scope, which only holds at the crate root, so the case files are
//! `include!`d here rather than mounted as `mod`s.
//!
//! Rules for a `plugin.rs` case file, because its tokens land in this module:
//! - no `use` items (they would collide across cases); rely on the glob imports
//!   below or write fully-qualified paths,
//! - every exported function name must be unique across the whole suite —
//!   hence the `roundtrip_<name>` convention.
//!
//! To add a test: `mkdir tests/<name>`, add `test.typ` + `plugin.rs`, then add
//! one `case!("<name>");` line below.

#![allow(unused_imports)]

typwire::initiate_protocol!();

use serde::{Deserialize, Serialize};
use typwire::*;

macro_rules! case {
    ($name:literal) => {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../../tests/",
            $name,
            "/plugin.rs"
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
// pair directly, as `int`, `array`, and `dictionary` do below.

// https://typst.app/docs/reference/foundations/
case!("foundations/arguments");
case!("foundations/auto");
case!("foundations/bool");
case!("foundations/bytes");
case!("foundations/datetime");
case!("foundations/decimal");
case!("foundations/duration");
case!("foundations/float");
case!("foundations/none");
case!("foundations/path");
case!("foundations/regex");
case!("foundations/str");
case!("foundations/symbol");
case!("foundations/type");
case!("foundations/version");

case!("foundations/int/i64");
case!("foundations/int/u8");

case!("foundations/array/vec");
case!("foundations/array/vec-deque");
case!("foundations/array/linked-list");
case!("foundations/array/btree-set");
case!("foundations/array/hash-set");
case!("foundations/array/fixed-size");
case!("foundations/array/tuple");

case!("foundations/dictionary/btree-map");
case!("foundations/dictionary/hash-map");
case!("foundations/dictionary/struct");

// https://typst.app/docs/reference/layout/
case!("layout/angle");
case!("layout/length");
case!("layout/ratio");
case!("layout/radius");

// https://typst.app/docs/reference/visualize/
case!("visualize/color");
case!("visualize/gradient");

// No dedicated reference page of their own, but grouped here since they only
// exist in service of the types above: `color-space` is the gradient/color
// `space` argument, `paint` is the color-or-gradient union used for fills
// and strokes, `center`/`stop` are gradient-only helper shapes.
case!("visualize/color-space");
case!("visualize/paint");
case!("visualize/center");
case!("visualize/stop");
