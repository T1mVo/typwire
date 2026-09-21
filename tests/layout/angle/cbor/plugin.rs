// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-angle-cbor")]
fn roundtrip_angle_cbor(value: Angle) -> Result<Angle, String> {
    Ok(value)
}
