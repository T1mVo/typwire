// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-gradient-cbor")]
fn roundtrip_gradient_cbor(value: Gradient) -> Result<Gradient, String> {
    Ok(value)
}
