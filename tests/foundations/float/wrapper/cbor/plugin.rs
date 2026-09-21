// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-float-wrapper-cbor")]
fn roundtrip_float_wrapper_cbor(value: typwire::Float) -> Result<typwire::Float, String> {
    Ok(value)
}
