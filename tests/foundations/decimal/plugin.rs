// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_decimal(value: Decimal) -> Result<Decimal, String> {
    Ok(value)
}
