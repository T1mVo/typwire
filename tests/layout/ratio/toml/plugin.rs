// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-ratio-toml")]
fn roundtrip_ratio_toml(value: Ratio) -> Result<Ratio, String> {
    Ok(value)
}
