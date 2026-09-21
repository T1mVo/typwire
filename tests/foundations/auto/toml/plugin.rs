// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-auto-toml")]
fn roundtrip_auto_toml(value: Auto) -> Result<Auto, String> {
    Ok(value)
}
