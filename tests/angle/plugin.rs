// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_angle(value: Angle) -> Result<Angle, String> {
    Ok(value)
}
