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
//! related (see `tests/rename/plugin.rs` or `tests/collections/plugin.rs`).
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

case!("unit");
case!("char");
case!("u8");
case!("collections");
case!("angle");
case!("length");
case!("length_radius");
case!("ratio");
case!("color");
case!("color_space");
case!("color_gradient");
case!("gradient");
case!("center");
case!("radius");
case!("stop");
case!("struct");
case!("option");
case!("any");
case!("one_of");
case!("rename");

// `tests/foundations/` mirrors Typst's own `foundations` module category
// (https://typst.app/docs/reference/foundations/) -- one dir per type name
// from that list that typwire actually supports.
case!("foundations/bool");
case!("foundations/bytes");
case!("foundations/datetime");
case!("foundations/duration");
case!("foundations/float");
case!("foundations/int");
case!("foundations/str");
case!("foundations/type");
case!("foundations/version");
