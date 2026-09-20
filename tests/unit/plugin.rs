// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_unit(value: ()) -> Result<(), String> {
    Ok(value)
}
