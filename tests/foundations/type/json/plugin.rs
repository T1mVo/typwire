// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-type-json")]
fn roundtrip_type_json(value: Type) -> Result<Type, String> {
    Ok(value)
}
