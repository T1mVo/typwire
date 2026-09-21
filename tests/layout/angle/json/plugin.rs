// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-angle-json")]
fn roundtrip_angle_json(value: Angle) -> Result<Angle, String> {
    Ok(value)
}
