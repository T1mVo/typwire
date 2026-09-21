// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_center(value: Center) -> Result<Center, String> {
    Ok(value)
}
