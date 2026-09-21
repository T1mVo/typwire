// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_stroke(value: Stroke) -> Result<Stroke, String> {
    Ok(value)
}
