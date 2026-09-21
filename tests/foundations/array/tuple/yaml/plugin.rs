// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-tuple-yaml")]
fn roundtrip_tuple_yaml(value: (i64, String, f64)) -> Result<(i64, String, f64), String> {
    Ok(value)
}
