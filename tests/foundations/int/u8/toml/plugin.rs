// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-u8-toml")]
fn roundtrip_u8_toml(value: u8) -> Result<u8, String> {
    Ok(value)
}
