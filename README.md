# typwire

Typed, Serde-backed data exchange for [Typst](https://typst.app/) WASM
plugins. Typwire preserves native Typst values across the CBOR plugin boundary
and generates protocol wrappers for ordinary Rust functions.

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
#import "@preview/typwire:0.2.0"

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

## Wire contract

Serde determines representation, but it does not automatically grant wire
access. User structs and enums must explicitly derive `Typwire` alongside
`Serialize` and `Deserialize`. This prevents an unrelated Serde type from
silently becoming part of a plugin ABI.

Supported Rust values include:

- booleans, signed and range-checked unsigned integers, floats, `char`, `String`,
  and unit
- `Option`, owned sequences, arrays, tuples, boxes, sets, and string-keyed
  `HashMap`/`BTreeMap`
- `Bytes` for a Typst byte string; `Vec<u8>` remains an ordinary array
- `Any` for dynamic values and `OneOf2` through `OneOf8` for ordered alternatives
- Typst angles, lengths, ratios, colors, gradients, datetimes, durations,
  versions, type objects, and their helper types

Typst integers are signed 64-bit values and dictionaries require string keys.
CBOR tags and values outside Typst's reliable CBOR subset are rejected.
`OneOfN` deserialization tries variants in declaration order, so overlapping
representations select the first match.

Borrowed Rust values (`&str`, `&[u8]`, and borrowed models), Typst content,
fractions, directions, relative lengths, and decimals are not supported.

`typwire.encode` and `typwire.decode` are available for manual integration.
`typwire.call` accepts positional arguments only and recursively preserves
supported native values inside arrays and dictionaries. Unknown Typwire tags and
type objects that cannot be reconstructed fail explicitly.
