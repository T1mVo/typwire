// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_auto(value: Auto) -> Result<Auto, String> {
    Ok(value)
}
