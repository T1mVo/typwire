// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_u8(value: u8) -> Result<u8, String> {
    Ok(value)
}
