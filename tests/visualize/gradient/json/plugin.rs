// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-gradient-json")]
fn roundtrip_gradient_json(value: Gradient) -> Result<Gradient, String> {
    Ok(value)
}
