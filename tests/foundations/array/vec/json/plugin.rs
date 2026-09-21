// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-vec-json")]
fn roundtrip_vec_json(value: Vec<i64>) -> Result<Vec<i64>, String> {
    Ok(value)
}
