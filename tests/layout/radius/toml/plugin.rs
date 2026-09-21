// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-radius-toml")]
fn roundtrip_radius_toml(value: Radius) -> Result<Radius, String> {
    Ok(value)
}
