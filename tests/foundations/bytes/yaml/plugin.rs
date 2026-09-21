// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-bytes-yaml")]
fn roundtrip_bytes_yaml(value: Bytes) -> Result<Bytes, String> {
    Ok(value)
}
