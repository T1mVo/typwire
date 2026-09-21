// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-duration-json")]
fn roundtrip_duration_json(value: Duration) -> Result<Duration, String> {
    Ok(value)
}
