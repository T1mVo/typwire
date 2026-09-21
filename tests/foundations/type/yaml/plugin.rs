// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-type-yaml")]
fn roundtrip_type_yaml(value: Type) -> Result<Type, String> {
    Ok(value)
}
