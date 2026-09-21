// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
// Exercises `#[typwire::export(format = "json")]`.
#[derive(Serialize, Deserialize, Typwire)]
struct FormatJsonPoint {
    x: f64,
    label: String,
}

#[typwire::export(format = "json")]
fn roundtrip_format_json_i64(value: i64) -> Result<i64, String> {
    Ok(value)
}

#[typwire::export(format = "json")]
fn roundtrip_format_json_length(value: Length) -> Result<Length, String> {
    Ok(value)
}

#[typwire::export(format = "json")]
fn roundtrip_format_json_bytes(value: Bytes) -> Result<Bytes, String> {
    Ok(value)
}

#[typwire::export(format = "json")]
fn roundtrip_format_json_any(value: Any) -> Result<Any, String> {
    Ok(value)
}

#[typwire::export(format = "json")]
fn roundtrip_format_json_struct(value: FormatJsonPoint) -> Result<FormatJsonPoint, String> {
    Ok(value)
}
