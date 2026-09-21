// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-gradient-toml")]
fn roundtrip_gradient_toml(value: Gradient) -> Result<Gradient, String> {
    Ok(value)
}
