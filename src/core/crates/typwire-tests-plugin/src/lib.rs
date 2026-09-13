//! Wasm plugin for the Typst roundtrip test suite.
//!
//! Every test in `<repo>/tests/<name>/` ships its own `plugin.rs` next to its
//! `test.typ`. Each of those files defines exactly one exported function
//!
//! ```ignore
//! #[typwire::export]
//! fn roundtrip_<name>(value: T) -> Result<T, String> { Ok(value) }
//! ```
//!
//! and is pulled in here with `case!`. `#[typwire::export]` expands to a
//! `#[wasm_func]` wrapper that needs the `initiate_protocol!()` items in scope,
//! which only holds at the crate root, so the case files are `include!`d here
//! rather than mounted as `mod`s.
//!
//! Rules for a `plugin.rs` case file, because its tokens land in this module:
//! - no `use` items (they would collide across cases); rely on the glob imports
//!   below or write fully-qualified paths,
//! - the exported function name must be unique across the whole suite — hence
//!   the `roundtrip_<name>` convention.
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

case!("angle");
case!("length");
case!("ratio");
case!("color");
case!("gradient");
case!("datetime");
case!("duration");
case!("version");
case!("type");
case!("struct");
case!("option");
case!("any");
case!("one_of");
case!("rename");
