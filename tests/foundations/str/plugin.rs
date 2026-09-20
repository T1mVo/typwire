// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_str(value: String) -> Result<String, String> {
    Ok(value)
}
