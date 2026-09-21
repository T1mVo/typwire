// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-int-wrapper-yaml")]
fn roundtrip_int_wrapper_yaml(value: typwire::Int) -> Result<typwire::Int, String> {
    Ok(value)
}
