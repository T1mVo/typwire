// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_type(value: Type) -> Result<Type, String> {
    Ok(value)
}
