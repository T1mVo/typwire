// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-decimal-cbor")]
fn roundtrip_decimal_cbor(value: Decimal) -> Result<Decimal, String> {
    Ok(value)
}
