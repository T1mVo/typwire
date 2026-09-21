// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-vec-yaml")]
fn roundtrip_vec_yaml(value: Vec<i64>) -> Result<Vec<i64>, String> {
    Ok(value)
}
