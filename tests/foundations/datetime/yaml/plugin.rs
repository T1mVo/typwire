// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-datetime-yaml")]
fn roundtrip_datetime_yaml(value: DateTime) -> Result<DateTime, String> {
    Ok(value)
}
