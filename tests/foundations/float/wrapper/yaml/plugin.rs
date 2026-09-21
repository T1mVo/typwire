// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-float-wrapper-yaml")]
fn roundtrip_float_wrapper_yaml(value: typwire::Float) -> Result<typwire::Float, String> {
    Ok(value)
}
