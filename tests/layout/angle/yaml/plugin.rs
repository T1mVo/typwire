// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-angle-yaml")]
fn roundtrip_angle_yaml(value: Angle) -> Result<Angle, String> {
    Ok(value)
}
