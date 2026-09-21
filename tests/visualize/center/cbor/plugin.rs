// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-center-cbor")]
fn roundtrip_center_cbor(value: Center) -> Result<Center, String> {
    Ok(value)
}
