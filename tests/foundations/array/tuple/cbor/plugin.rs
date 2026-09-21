// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-tuple-cbor")]
fn roundtrip_tuple_cbor(value: (i64, String, f64)) -> Result<(i64, String, f64), String> {
    Ok(value)
}
