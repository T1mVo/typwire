// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-u8-cbor")]
fn roundtrip_u8_cbor(value: u8) -> Result<u8, String> {
    Ok(value)
}
