// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "stress-bytes-json")]
fn stress_bytes_json(value: Bytes) -> Result<Bytes, String> {
    Ok(value)
}
