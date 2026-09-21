// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-i64-json")]
fn roundtrip_i64_json(value: i64) -> Result<i64, String> {
    Ok(value)
}
