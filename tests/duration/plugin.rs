// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_duration(value: Duration) -> Result<Duration, String> {
    Ok(value)
}
