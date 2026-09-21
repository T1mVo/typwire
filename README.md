# typwire

Typed, Serde-backed data exchange for [Typst](https://typst.app/) WASM
plugins. Typwire preserves native Typst values across the plugin boundary and
generates protocol wrappers for ordinary Rust functions. CBOR is the default
wire format on both sides; JSON, YAML, and TOML are available per export/call
(see *Wire formats* below).

The Typst and Rust package versions must match exactly.

## Usage

Add `typwire` and Serde to a Rust plugin:

```sh
cargo add typwire
cargo add serde --features derive
```

```rust
use serde::{Deserialize, Serialize};
use typwire::{Any, Length, OneOf3, Typwire, initiate_protocol};

initiate_protocol!();

#[derive(Serialize, Deserialize, Typwire)]
struct Options {
    label: String,
    scale: f64,
}

#[derive(Serialize, Deserialize, Typwire)]
struct Report {
    text: String,
    width: Length,
    metadata: Any,
}

#[typwire::export]
fn measure(
    value: OneOf3<i64, f64, String>,
    options: Options,
    fallback: Option<Length>,
    metadata: Any,
) -> Result<Report, String> {
    let value = match value {
        OneOf3::First(value) => value.to_string(),
        OneOf3::Second(value) => value.to_string(),
        OneOf3::Third(value) => value,
    };

    Ok(Report {
        text: format!("{}{}", options.label, value),
        width: fallback.unwrap_or_else(|| Length::new(12.0 * options.scale)),
        metadata,
    })
}
```

Call it from Typst with separately encoded positional arguments:

```typ
#import "@preview/typwire:0.3.0"

#let plugin = plugin("plugin.wasm")
#let report = typwire.call(
  plugin.measure,
  42,
  (label: "Value: ", scale: 1.5),
  12pt,
  (source: "example", enabled: true),
)

#assert(report.text == "Value: 42")
#assert(report.width == 12pt)
```

`#[typwire::export]` requires owned, identifier-named parameters and an explicit
`Result<T, E>` return, where all parameters and `T` implement `Typwire` and `E`
implements `Display`. Decode errors identify the function and argument. Rust
errors and result-encoding errors are surfaced by Typst as plugin panics.

By default the wasm export takes the Rust function's name. Pass
`#[typwire::export(rename = "other-name")]` to choose a different one; the name
need not be a valid Rust identifier. A non-identifier export is reached from
Typst through `dictionary(plugin).at("other-name")`.

## Wire formats

CBOR is the default everywhere. Pass `#[typwire::export(format = "json")]`
(also `"yaml"`/`"toml"`) to decode/encode that export's arguments and result
through a different format instead — each behind its own crate feature
(`json`/`yaml`/`toml`, off by default, `cargo add typwire --features json`):

```rust
#[typwire::export(format = "json")]
fn measure_json(options: Options) -> Result<Report, String> {
    /* .. */
}
```

Call it from Typst through the matching namespace instead of `typwire.call`:

```typ
#let report = typwire.json.call(plugin.measure_json, (label: "Value: ", scale: 1.5))
```

`typwire.cbor`/`typwire.json`/`typwire.yaml`/`typwire.toml` each expose the
same `call`/`encode`/`decode` shape; `typwire.call`/`typwire.encode`/
`typwire.decode` (no namespace) are aliases for the `cbor` ones. A plugin
export and the Typst call site must agree on the same format — nothing
checks this for you, the same way argument types on either side already
have to agree.

JSON/YAML have no native way to tell an integer apart from a float the way
CBOR/TOML can, and none of the three have a native byte-string type; both
are handled transparently (see `CLAUDE.md` if you're curious how), so this
mostly isn't something you need to think about. Two exceptions: TOML
requires a struct/dictionary-shaped value at the top level of a call or
export (a bare `i64` or `Vec<T>` argument fails to encode; TOML's own
grammar requires a root table, matching Typst's own `toml.encode`), and a
stroke's tiling-pattern paint isn't representable in any format (see
*Wire contract* below).

## Wire contract

Serde determines representation, but it does not automatically grant wire
access. User structs and enums must explicitly derive `Typwire` alongside
`Serialize` and `Deserialize`. This prevents an unrelated Serde type from
silently becoming part of a plugin ABI.

Supported Rust values include:

- booleans, `i64`, `f64`, `char`, `String`, and unit — matching Typst's own
  integer and float types; no other Rust integer or float width is `Typwire`
- `Option` for Typst's `none`, `Auto` for Typst's `auto`; owned sequences,
  arrays, tuples, boxes, sets, and string-keyed `HashMap`/`BTreeMap`
- `Bytes` for a Typst byte string; `Vec<u8>` remains an ordinary array
- `Any` for dynamic values and `OneOf2` through `OneOf8` for ordered alternatives
- Typst angles, lengths, ratios, alignments, colors, gradients, strokes,
  datetimes, durations, versions, type objects, paths, symbols, labels,
  captured arguments, and their helper types
- `Regex` for a Typst regular expression, backed by a compiled
  [`regex-lite`](https://docs.rs/regex-lite) pattern, behind this crate's
  optional `regex` feature
- `Decimal` for a Typst fixed-point decimal number, backed by
  [`rust_decimal`](https://docs.rs/rust_decimal) — the same crate Typst's
  own `decimal` type wraps internally — behind this crate's optional
  `decimal` feature

Both `Regex` and `Decimal` only exist when their feature is enabled: without
it, referencing the type is a compile error, not a value that builds fine
and only fails when decoded.

Typst integers are signed 64-bit values and dictionaries require string keys.
CBOR tags and values outside Typst's reliable CBOR subset are rejected.
`OneOfN` deserialization tries variants in declaration order, so overlapping
representations select the first match.

Borrowed Rust values (`&str`, `&[u8]`, and borrowed models), Typst content,
fractions, directions, and relative lengths are not supported. Stroke paints
are limited to solid colors and gradients; Typst's tiling patterns have no
`Typwire` representation.

`typwire.encode`/`typwire.decode` (and their per-format counterparts) are
available for manual integration. `typwire.call` accepts positional
arguments only and recursively preserves supported native values inside
arrays and dictionaries. Unknown Typwire tags and type objects that cannot
be reconstructed fail explicitly.
