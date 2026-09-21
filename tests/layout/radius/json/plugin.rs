// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-radius-json")]
fn roundtrip_radius_json(value: Radius) -> Result<Radius, String> {
    Ok(value)
}
