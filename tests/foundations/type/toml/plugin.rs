// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-type-toml")]
fn roundtrip_type_toml(value: Type) -> Result<Type, String> {
    Ok(value)
}
