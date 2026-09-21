// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-auto-cbor")]
fn roundtrip_auto_cbor(value: Auto) -> Result<Auto, String> {
    Ok(value)
}
