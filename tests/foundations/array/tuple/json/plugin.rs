// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-tuple-json")]
fn roundtrip_tuple_json(value: (i64, String, f64)) -> Result<(i64, String, f64), String> {
    Ok(value)
}
