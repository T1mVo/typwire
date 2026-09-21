// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-duration-yaml")]
fn roundtrip_duration_yaml(value: Duration) -> Result<Duration, String> {
    Ok(value)
}
