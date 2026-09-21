// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-bool-yaml")]
fn roundtrip_bool_yaml(value: bool) -> Result<bool, String> {
    Ok(value)
}
