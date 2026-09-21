// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-bytes-json")]
fn roundtrip_bytes_json(value: Bytes) -> Result<Bytes, String> {
    Ok(value)
}
