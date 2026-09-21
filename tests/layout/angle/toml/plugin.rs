// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-angle-toml")]
fn roundtrip_angle_toml(value: Angle) -> Result<Angle, String> {
    Ok(value)
}
