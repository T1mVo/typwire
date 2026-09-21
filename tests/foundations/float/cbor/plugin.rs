// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-float-cbor")]
fn roundtrip_float_cbor(value: f64) -> Result<f64, String> {
    Ok(value)
}
