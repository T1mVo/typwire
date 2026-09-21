// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-radius-yaml")]
fn roundtrip_radius_yaml(value: Radius) -> Result<Radius, String> {
    Ok(value)
}
