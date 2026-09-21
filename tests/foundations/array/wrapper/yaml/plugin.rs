// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-array-wrapper-yaml")]
fn roundtrip_array_wrapper_yaml(value: typwire::Array<i64>) -> Result<typwire::Array<i64>, String> {
    Ok(value)
}
