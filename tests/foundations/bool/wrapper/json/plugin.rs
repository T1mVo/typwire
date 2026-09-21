// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-bool-wrapper-json")]
fn roundtrip_bool_wrapper_json(value: typwire::Bool) -> Result<typwire::Bool, String> {
    Ok(value)
}
