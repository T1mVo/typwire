// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-array-wrapper-json")]
fn roundtrip_array_wrapper_json(value: typwire::Array<i64>) -> Result<typwire::Array<i64>, String> {
    Ok(value)
}
