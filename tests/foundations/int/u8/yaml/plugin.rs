// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-u8-yaml")]
fn roundtrip_u8_yaml(value: u8) -> Result<u8, String> {
    Ok(value)
}
