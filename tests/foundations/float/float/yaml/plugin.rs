// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-float-yaml")]
fn roundtrip_float_yaml(value: f64) -> Result<f64, String> {
    Ok(value)
}
