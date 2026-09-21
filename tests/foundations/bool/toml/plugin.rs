// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-bool-toml")]
fn roundtrip_bool_toml(value: bool) -> Result<bool, String> {
    Ok(value)
}
