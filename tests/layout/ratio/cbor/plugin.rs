// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-ratio-cbor")]
fn roundtrip_ratio_cbor(value: Ratio) -> Result<Ratio, String> {
    Ok(value)
}
