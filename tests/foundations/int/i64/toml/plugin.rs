// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-i64-toml")]
fn roundtrip_i64_toml(value: i64) -> Result<i64, String> {
    Ok(value)
}
