// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-vec-cbor")]
fn roundtrip_vec_cbor(value: Vec<i64>) -> Result<Vec<i64>, String> {
    Ok(value)
}
