// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_length_radius(value: LengthRadius) -> Result<LengthRadius, String> {
    Ok(value)
}
