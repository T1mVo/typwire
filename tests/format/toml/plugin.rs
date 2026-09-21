// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
// Exercises `#[typwire::export(format = "toml")]`. No bare-`i64` case here
// (unlike `tests/format/json`/`yaml`): TOML's own grammar requires a
// top-level table, so a bare scalar argument would fail to encode -- see
// `tests/format/toml/test.typ`.
#[derive(Serialize, Deserialize, Typwire)]
struct FormatTomlPoint {
    x: f64,
    label: String,
}

#[typwire::export(format = "toml")]
fn roundtrip_format_toml_length(value: Length) -> Result<Length, String> {
    Ok(value)
}

#[typwire::export(format = "toml")]
fn roundtrip_format_toml_bytes(value: Bytes) -> Result<Bytes, String> {
    Ok(value)
}

#[typwire::export(format = "toml")]
fn roundtrip_format_toml_any(value: Any) -> Result<Any, String> {
    Ok(value)
}

#[typwire::export(format = "toml")]
fn roundtrip_format_toml_struct(value: FormatTomlPoint) -> Result<FormatTomlPoint, String> {
    Ok(value)
}
