// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_datetime(value: DateTime) -> Result<DateTime, String> {
    Ok(value)
}
