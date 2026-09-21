// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-vec-toml")]
fn roundtrip_vec_toml(value: Vec<i64>) -> Result<Vec<i64>, String> {
    Ok(value)
}
