// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-duration-toml")]
fn roundtrip_duration_toml(value: Duration) -> Result<Duration, String> {
    Ok(value)
}
