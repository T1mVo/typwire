// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-datetime-json")]
fn roundtrip_datetime_json(value: DateTime) -> Result<DateTime, String> {
    Ok(value)
}
