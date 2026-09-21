// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-datetime-cbor")]
fn roundtrip_datetime_cbor(value: DateTime) -> Result<DateTime, String> {
    Ok(value)
}
