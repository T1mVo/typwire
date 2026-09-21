// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-none-yaml")]
fn roundtrip_none_yaml(value: Option<Length>) -> Result<Option<Length>, String> {
    Ok(value)
}
