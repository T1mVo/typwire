// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-i64-yaml")]
fn roundtrip_i64_yaml(value: i64) -> Result<i64, String> {
    Ok(value)
}
