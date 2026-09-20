// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_radius(value: Radius) -> Result<Radius, String> {
    Ok(value)
}
