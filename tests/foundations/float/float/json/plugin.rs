// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-float-json")]
fn roundtrip_float_json(value: f64) -> Result<f64, String> {
    Ok(value)
}
