# typwire

Typed data exchange between Typst and (Rust) WASM plugins.

## Installation

Add the Rust crate to a plugin:

```sh
cargo add typwire
```

Add the Typst package to a document:

```typ
#import "@preview/typwire:0.3.0"
```

## Quick Start

Export a plugin function from Rust:

```rust
use typwire::{Length, initiate_protocol};

initiate_protocol!();

#[typwire::export]
fn measure(label: String, scale: f64) -> Result<(String, Length), String> {
    Ok((format!("{label}{scale}"), Length::new(12.0 * scale)))
}
```

Call the plugin function from Typst:

```typ
#import "@preview/typwire:0.3.0"

#let plugin = plugin("plugin.wasm")
#let (text, width) = typwire.call(plugin.measure, "Value: ", 1.5)

#assert(text == "Value: 1.5")
#assert(width == 18pt)
```

## Reference

### `typwire`

Alias for `typwire.cbor` below — CBOR is the default wire format on both
sides, the same one `#[typwire::export]` picks when no `format` is given.

```typ
/// Calls a plugin function: encodes each positional argument
/// independently, invokes the export, and decodes its single return value,
/// using CBOR. Named arguments are not supported.
/// -> any
#let call(function, ..args) = { /* .. */ }
```

```typ
/// Encodes a value into CBOR bytes.
/// -> bytes
#let encode(value) = { /* .. */ }
```

```typ
/// Decodes CBOR bytes and reconstructs supported native Typst values.
/// -> any
#let decode(data) = { /* .. */ }
```

### `typwire.debug`

No serialization at all — `encode` stops at the shared dictionary/array/
primitive tree every other namespace's `encode` would otherwise hand to a
native format encoder, and `decode` reconstructs from that same tree. No
`call`: there's no `bytes`/`str` here to hand a plugin. Useful for
inspecting or testing the wire shape Typwire produces without going through
an actual format or a plugin round trip.

```typ
/// Encodes a value into the format-agnostic tree, with no serialization.
/// -> any
#let encode(value) = { /* .. */ }
```

```typ
/// The mirror of `encode`: reconstructs native Typst values from a tree
/// that was never serialized.
/// -> any
#let decode(data) = { /* .. */ }
```

<details>
<summary><code>typwire.cbor</code></summary>

```typ
/// Encodes a value into CBOR bytes.
/// -> bytes
#let encode(value) = { /* .. */ }
```

```typ
/// Decodes CBOR bytes and reconstructs supported native Typst values.
/// -> any
#let decode(data) = { /* .. */ }
```

```typ
/// Calls a plugin function with positional arguments, using CBOR.
/// -> any
#let call(function, ..args) = { /* .. */ }
```

</details>

<details>
<summary><code>typwire.json</code></summary>

```typ
/// Encodes a value into a JSON string. JSON has no native byte-string
/// type, so any `bytes` value is rewritten into a tagged array first.
/// -> str
#let encode(value) = { /* .. */ }
```

```typ
/// Decodes a JSON string or bytes and reconstructs supported native Typst
/// values.
/// -> any
#let decode(data) = { /* .. */ }
```

```typ
/// Calls a plugin function with positional arguments, using JSON.
/// -> any
#let call(function, ..args) = { /* .. */ }
```

</details>

<details>
<summary><code>typwire.yaml</code></summary>

```typ
/// Encodes a value into a YAML string. YAML has no native byte-string
/// type, so any `bytes` value is rewritten into a tagged array first.
/// -> str
#let encode(value) = { /* .. */ }
```

```typ
/// Decodes a YAML string or bytes and reconstructs supported native Typst
/// values.
/// -> any
#let decode(data) = { /* .. */ }
```

```typ
/// Calls a plugin function with positional arguments, using YAML.
/// -> any
#let call(function, ..args) = { /* .. */ }
```

</details>

<details>
<summary><code>typwire.toml</code></summary>

```typ
/// Encodes a value into a TOML string. TOML has no native byte-string type
/// and no `none`/null, and its grammar requires a top-level table -- all
/// handled transparently by wrapping and rewriting the tree before it
/// reaches TOML's own encoder.
/// -> str
#let encode(value) = { /* .. */ }
```

```typ
/// Decodes a TOML string or bytes and reconstructs supported native Typst
/// values.
/// -> any
#let decode(data) = { /* .. */ }
```

```typ
/// Calls a plugin function with positional arguments, using TOML.
/// -> any
#let call(function, ..args) = { /* .. */ }
```

</details>

## Supported types

Every Typst value with a native CBOR/JSON primitive (`int`, `float`, `bool`,
`str`, `none`, `array`, `dictionary`) round-trips directly, with no extra
wire structure. Everything else is sent as a tagged dictionary carrying a
`typwire-type` field, reconstructed back into the real Typst value on
decode. A handful of those types have no other way to read their data back
out of Typst at all and are encoded by parsing Typst's own `repr()` output
instead of calling an accessor — marked below.

| Typst type | Rust type(s) | Wire shape | Uses `repr()` |
| --- | --- | --- | --- |
| `int` | `i64`, `Int` | native | |
| `float` | `f64`, `Float` | native | |
| `bool` | `bool`, `Bool` | native | |
| `str` | `String`, `Str` | native | |
| `none` | `Option::None` | native¹ | |
| `array` | `Vec<T>`, `VecDeque<T>`, `LinkedList<T>`, `BTreeSet<T>`, `HashSet<T>`, `[T; N]`, tuples, `Array<T>` | native | |
| `dictionary` | `BTreeMap<String, T>`, `HashMap<String, T>`, derived structs, `Dictionary<T>` | native | |
| `bytes` | `Bytes` (`Vec<u8>` stays a plain array) | native (CBOR); tagged array² (JSON/YAML/TOML) | |
| `auto` | `Auto` | tagged | |
| `type` | `Type` | tagged | |
| `label` | `Label` | tagged | |
| `version` | `Version` | tagged | component count |
| `symbol` | `Symbol` | tagged | ✓ |
| `path` | `Path` | tagged | ✓ |
| `regex` (feature `regex`) | `Regex` | tagged | ✓ |
| `decimal` (feature `decimal`) | `Decimal` | tagged | |
| `datetime` | `DateTime` | tagged | |
| `duration` | `Duration` | tagged | |
| `arguments` | `Arguments` | tagged | |
| `angle` | `Angle` | tagged | |
| `length` | `Length` | tagged | |
| `ratio` | `Ratio` | tagged | |
| `alignment` | `Alignment` | tagged | |
| corner radius dictionary | `Radius` | plain dictionary³ | |
| `color` (and its 8 color spaces) | `Color` | tagged | |
| `gradient` (linear/radial/conic, stops) | `Gradient` | tagged | |
| `stroke` | `Stroke` | tagged, built from the wire shapes above | |

¹ TOML has no `none`/null of its own, so its format layer alone rewrites
`none` into a tagged dictionary behind the scenes; every other format keeps
it native.
² JSON/YAML/TOML have no byte-string type, so their format layer rewrites
`bytes` into a tagged array of byte integers; CBOR keeps its native byte
string.
³ Typst represents a corner-radius dictionary as an ordinary dictionary too,
not a distinct runtime type, so this is the one type with no `typwire-type`
tag at all.

Two more mechanisms sit outside this table: `Any` holds any supported value
dynamically (reconstructing native types by their tag rather than falling
back to a plain dictionary), and `OneOf2` through `OneOf8` accept an ordered
list of alternative shapes, trying each in declaration order.

Not supported: borrowed values (`&str`, `&[u8]`, borrowed models), Typst
`content`, `function`, fractions, directions, relative lengths, and a
stroke's tiling-pattern paint.

## Wire shapes

The actual Typst dictionary each tagged type above round-trips as — the
same shape `typwire.debug.encode` returns. Every one carries a
`typwire-type` string tag except the corner radius dictionary (the one
exception — Typst itself has no distinct runtime type for it either).
Native primitives (`int`, `float`, `bool`, `str`, `none`, `array`,
`dictionary`) aren't repeated here; they pass through untagged.

#### Foundations

##### Auto

```typ
(typwire-type: "auto")
```

##### Type

```typ
(typwire-type: "type", ty: "int")
```

##### Label

```typ
(typwire-type: "label", name: "intro")
```

##### Version

```typ
(typwire-type: "version", components: (1, 2, 3))
```

**Note:** `version` uses `repr()` to determine the component count, whose output is unstable and might change any time. Use this type with caution.

##### Symbol

```typ
(
  typwire-type: "symbol",
  default: "→",
  variants: ((modifiers: "double", glyph: "⇒"),),
)
```

**Note:** `symbol` uses `repr()` to access the variants, whose output is unstable and might change any time. Use this type with caution. 

##### Path

```typ
(typwire-type: "path", path: "assets/logo.svg")
```

**Note:** `path` uses `repr()` to recover its text, whose output is unstable and might change any time. Use
this type with caution.

##### Regex

```typ
(typwire-type: "regex", pattern: "\d+")
```

**Note:** `regex` uses `repr()` to recover its pattern, whose output is unstable and might change any time. Use
this type with caution. Requires the `regex` feature on the Rust side.

##### Decimal

```typ
(typwire-type: "decimal", decimal: "3.14")
```

**Note:** Requires the `decimal` feature on the Rust side.

##### Datetime

```typ
(
  typwire-type: "datetime",
  year: 2024,
  month: 1,
  day: 1,
  hour: none,
  minute: none,
  second: none,
)
```

##### Duration

```typ
(typwire-type: "duration", seconds: 3600.0)
```

##### Arguments

```typ
(typwire-type: "arguments", pos: (1, "two"), named: (flag: true))
```

##### Bytes

```typ
(typwire-type: "bytes", data: (1, 2, 3))
```

**Note:** Only used by JSON/YAML/TOML; CBOR keeps a native byte string.

##### None

```typ
(typwire-type: "none")
```

**Note:** Only used by TOML; every other format keeps a native `null`.

#### Layout

##### Angle

```typ
(typwire-type: "angle", radians: 1.5707963267948966)
```

##### Length

```typ
(typwire-type: "length", points: 28.35)
```

##### Ratio

```typ
(typwire-type: "ratio", ratio: 0.5)
```

##### Alignment

```typ
(typwire-type: "alignment", x: "left", y: none)
```

#### Visualize

##### Color

```typ
(typwire-type: "color-luma", lightness: 100%, alpha: 100%)
```

```typ
(typwire-type: "color-rgb", r: 1.0, g: 0.0, b: 0.0, alpha: 1.0)
```

```typ
(typwire-type: "color-linear-rgb", r: 1.0, g: 0.0, b: 0.0, alpha: 1.0)
```

```typ
(typwire-type: "color-cmyk", cyan: 0%, magenta: 100%, yellow: 100%, key: 0%)
```

```typ
(
  typwire-type: "color-hsl",
  hue: (typwire-type: "angle", radians: 0.0),
  saturation: 100%, lightness: 50%, alpha: 100%,
)
```

```typ
(
  typwire-type: "color-hsv",
  hue: (typwire-type: "angle", radians: 0.0),
  saturation: 100%, value: 100%, alpha: 100%,
)
```

```typ
(typwire-type: "color-oklab", lightness: 63%, a: 0.22, b: 0.13, alpha: 100%)
```

```typ
(
  typwire-type: "color-oklch",
  lightness: 63%, chroma: 0.26,
  hue: (typwire-type: "angle", radians: 0.72),
  alpha: 100%,
)
```

##### Gradient

```typ
(
  typwire-type: "gradient-linear",
  stops: ((color: (typwire-type: "color-rgb", r: 1.0, g: 0.0, b: 0.0, alpha: 1.0), offset: 0%),),
  angle: (typwire-type: "angle", radians: 0.0),
  space: "rgb",
)
```

```typ
(
  typwire-type: "gradient-radial",
  stops: (/* stop, stop, ... */),
  center: (x: 50%, y: 50%),
  radius: 50%,
  focal-center: (x: 50%, y: 50%),
  focal-radius: 0%,
  space: "rgb",
)
```

```typ
(
  typwire-type: "gradient-conic",
  stops: (/* stop, stop, ... */),
  angle: (typwire-type: "angle", radians: 0.0),
  center: (x: 50%, y: 50%),
  space: "rgb",
)
```

##### Stroke

```typ
(
  typwire-type: "stroke",
  paint: (typwire-type: "color-rgb", r: 0.0, g: 0.0, b: 0.0, alpha: 1.0),
  thickness: (typwire-type: "length", points: 1.0),
  cap: "round",
  join: "round",
  dash: none,
  miter-limit: 4.0,
)
```

## Format performance

If no specific format is desired, CBOR is the most effiecint option and should be preferred.

| Format | Relative speed |
| :-: | :-: |
| CBOR | 1x (baseline) |
| JSON | ~1.4x – 1.5x |
| TOML | ~2.1x – 2.2x |
| YAML | ~4.9x – 6.2x |

## Version compatibility

The Rust crate and the Typst package are versioned together and must be on
the **exact same version**. There is no compatibility guarantee across a version mismatch.

## License

`typwire` is available under the MIT OR Apache-2.0 license. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.
