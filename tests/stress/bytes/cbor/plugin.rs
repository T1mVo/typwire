// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "stress-bytes-cbor")]
fn stress_bytes_cbor(value: Bytes) -> Result<Bytes, String> {
    Ok(value)
}
