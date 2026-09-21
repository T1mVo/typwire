// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-u8-json")]
fn roundtrip_u8_json(value: u8) -> Result<u8, String> {
    Ok(value)
}
