// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-stop-cbor")]
fn roundtrip_stop_cbor(value: Stop) -> Result<Stop, String> {
    Ok(value)
}
