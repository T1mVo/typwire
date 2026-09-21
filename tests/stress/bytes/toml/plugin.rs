// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "stress-bytes-toml")]
fn stress_bytes_toml(value: Bytes) -> Result<Bytes, String> {
    Ok(value)
}
