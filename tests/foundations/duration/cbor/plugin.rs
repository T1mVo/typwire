// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-duration-cbor")]
fn roundtrip_duration_cbor(value: Duration) -> Result<Duration, String> {
    Ok(value)
}
