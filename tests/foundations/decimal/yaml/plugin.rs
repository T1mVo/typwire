// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-decimal-yaml")]
fn roundtrip_decimal_yaml(value: Decimal) -> Result<Decimal, String> {
    Ok(value)
}
