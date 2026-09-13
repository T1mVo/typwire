// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_ratio(value: Ratio) -> Result<Ratio, String> {
    Ok(value)
}
