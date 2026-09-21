// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-datetime-toml")]
fn roundtrip_datetime_toml(value: DateTime) -> Result<DateTime, String> {
    Ok(value)
}
