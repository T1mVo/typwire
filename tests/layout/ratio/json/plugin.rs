// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-ratio-json")]
fn roundtrip_ratio_json(value: Ratio) -> Result<Ratio, String> {
    Ok(value)
}
