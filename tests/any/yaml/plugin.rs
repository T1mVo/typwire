// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-any-yaml")]
fn roundtrip_any_yaml(value: Any) -> Result<Any, String> {
    Ok(value)
}
