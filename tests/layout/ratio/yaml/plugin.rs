// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-ratio-yaml")]
fn roundtrip_ratio_yaml(value: Ratio) -> Result<Ratio, String> {
    Ok(value)
}
