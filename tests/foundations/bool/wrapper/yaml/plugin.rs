// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-bool-wrapper-yaml")]
fn roundtrip_bool_wrapper_yaml(value: typwire::Bool) -> Result<typwire::Bool, String> {
    Ok(value)
}
