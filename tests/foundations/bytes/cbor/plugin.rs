// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-bytes-cbor")]
fn roundtrip_bytes_cbor(value: Bytes) -> Result<Bytes, String> {
    Ok(value)
}
