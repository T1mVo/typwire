// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_bytes(value: Bytes) -> Result<Bytes, String> {
    Ok(value)
}
