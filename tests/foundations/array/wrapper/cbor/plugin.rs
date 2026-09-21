// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-array-wrapper-cbor")]
fn roundtrip_array_wrapper_cbor(value: typwire::Array<i64>) -> Result<typwire::Array<i64>, String> {
    Ok(value)
}
