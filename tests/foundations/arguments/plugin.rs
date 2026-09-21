// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_arguments(value: Arguments) -> Result<Arguments, String> {
    Ok(value)
}
