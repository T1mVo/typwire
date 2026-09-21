// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-any-toml")]
fn roundtrip_any_toml(value: Any) -> Result<Any, String> {
    Ok(value)
}
