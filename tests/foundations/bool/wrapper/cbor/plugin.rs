// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-bool-wrapper-cbor")]
fn roundtrip_bool_wrapper_cbor(value: typwire::Bool) -> Result<typwire::Bool, String> {
    Ok(value)
}
