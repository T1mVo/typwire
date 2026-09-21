// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-i64-cbor")]
fn roundtrip_i64_cbor(value: i64) -> Result<i64, String> {
    Ok(value)
}
