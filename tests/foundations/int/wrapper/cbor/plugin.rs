// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-int-wrapper-cbor")]
fn roundtrip_int_wrapper_cbor(value: typwire::Int) -> Result<typwire::Int, String> {
    Ok(value)
}
