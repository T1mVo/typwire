// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-auto-yaml")]
fn roundtrip_auto_yaml(value: Auto) -> Result<Auto, String> {
    Ok(value)
}
