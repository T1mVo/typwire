// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-decimal-json")]
fn roundtrip_decimal_json(value: Decimal) -> Result<Decimal, String> {
    Ok(value)
}
