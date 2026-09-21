// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-array-wrapper-toml")]
fn roundtrip_array_wrapper_toml(value: typwire::Array<i64>) -> Result<typwire::Array<i64>, String> {
    Ok(value)
}
