// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_bool(value: bool) -> Result<bool, String> {
    Ok(value)
}
