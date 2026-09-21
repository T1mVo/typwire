// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_none(value: Option<Length>) -> Result<Option<Length>, String> {
    Ok(value)
}
