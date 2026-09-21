// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-str-toml")]
fn roundtrip_str_toml(value: String) -> Result<String, String> {
    Ok(value)
}
