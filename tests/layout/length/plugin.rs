// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_length(value: Length) -> Result<Length, String> {
    Ok(value)
}
