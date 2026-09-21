// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-tuple-toml")]
fn roundtrip_tuple_toml(value: (i64, String, f64)) -> Result<(i64, String, f64), String> {
    Ok(value)
}
