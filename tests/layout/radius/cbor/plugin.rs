// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-radius-cbor")]
fn roundtrip_radius_cbor(value: Radius) -> Result<Radius, String> {
    Ok(value)
}
