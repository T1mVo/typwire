// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-gradient-yaml")]
fn roundtrip_gradient_yaml(value: Gradient) -> Result<Gradient, String> {
    Ok(value)
}
