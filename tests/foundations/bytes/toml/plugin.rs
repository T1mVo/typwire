// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-bytes-toml")]
fn roundtrip_bytes_toml(value: Bytes) -> Result<Bytes, String> {
    Ok(value)
}
