// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
// Exercises `#[typwire::export(format = "yaml")]`.
#[derive(Serialize, Deserialize, Typwire)]
struct FormatYamlPoint {
    x: f64,
    label: String,
}

#[typwire::export(format = "yaml")]
fn roundtrip_format_yaml_i64(value: i64) -> Result<i64, String> {
    Ok(value)
}

#[typwire::export(format = "yaml")]
fn roundtrip_format_yaml_length(value: Length) -> Result<Length, String> {
    Ok(value)
}

#[typwire::export(format = "yaml")]
fn roundtrip_format_yaml_bytes(value: Bytes) -> Result<Bytes, String> {
    Ok(value)
}

#[typwire::export(format = "yaml")]
fn roundtrip_format_yaml_any(value: Any) -> Result<Any, String> {
    Ok(value)
}

#[typwire::export(format = "yaml")]
fn roundtrip_format_yaml_struct(value: FormatYamlPoint) -> Result<FormatYamlPoint, String> {
    Ok(value)
}
