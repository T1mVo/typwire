//! Wasm plugin for the Typst roundtrip test suite.
//!
//! Every test in `<repo>/tests/<name>/` (`<name>` may nest, e.g.
//! `foundations/bool`) ships **two** sibling cases, one per wire format --
//! `tests/<name>/{cbor,json}/`, each its own `plugin.rs` next to its
//! `test.typ`. `plugin.rs` usually has one exported function
//!
//! ```ignore
//! #[typwire::export(rename = "roundtrip-<name>-<format>")]
//! fn roundtrip_<name>_<format>(value: T) -> Result<T, String> { Ok(value) }
//! ```
//!
//! To add a test: `mkdir -p tests/<name>/{cbor,json}`, add a
//! `test.typ` + `plugin.rs` pair to each, then add one `case!("<name>");`
//! line below -- it expands to both `include!`s.

#![allow(unused_imports)]

typwire::initiate_protocol!();

use serde::{Deserialize, Serialize};
use typwire::*;

/// Pulls in both format variants of one test: `tests/<name>/{cbor,json}/plugin.rs`.
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
    };
}

case!("any");
case!("one-of");
case!("rename");
case!("box");
case!("unit");
case!("char");

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

case!("layout/alignment");
case!("layout/angle");
case!("layout/length");
case!("layout/ratio");
case!("layout/radius");

case!("visualize/color");
case!("visualize/gradient");
case!("visualize/stroke");

case!("visualize/color-space");
case!("visualize/paint");
case!("visualize/center");
case!("visualize/stop");

case!("stress/bytes");
case!("stress/multi-arg");
case!("stress/many-objects");

case!("stress/warmup");
