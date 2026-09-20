// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_float(value: f64) -> Result<f64, String> {
    Ok(value)
}
