// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_stop(value: Stop) -> Result<Stop, String> {
    Ok(value)
}
