// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_gradient(value: Gradient) -> Result<Gradient, String> {
    Ok(value)
}
