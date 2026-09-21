// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-float-wrapper-json")]
fn roundtrip_float_wrapper_json(value: typwire::Float) -> Result<typwire::Float, String> {
    Ok(value)
}
