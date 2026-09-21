// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-decimal-toml")]
fn roundtrip_decimal_toml(value: Decimal) -> Result<Decimal, String> {
    Ok(value)
}
