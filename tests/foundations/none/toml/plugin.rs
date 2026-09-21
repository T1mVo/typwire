// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-none-toml")]
fn roundtrip_none_toml(value: Option<Length>) -> Result<Option<Length>, String> {
    Ok(value)
}
