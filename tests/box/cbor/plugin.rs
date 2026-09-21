// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-box-cbor")]
fn roundtrip_box_cbor(value: Box<i64>) -> Result<Box<i64>, String> {
    Ok(value)
}
