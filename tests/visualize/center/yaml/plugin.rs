// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-center-yaml")]
fn roundtrip_center_yaml(value: Center) -> Result<Center, String> {
    Ok(value)
}
