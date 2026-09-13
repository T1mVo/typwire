// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_any(value: Any) -> Result<Any, String> {
    Ok(value)
}
