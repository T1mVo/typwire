// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-float-toml")]
fn roundtrip_float_toml(value: f64) -> Result<f64, String> {
    Ok(value)
}
