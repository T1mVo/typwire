// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-arguments-yaml")]
fn roundtrip_arguments_yaml(value: Arguments) -> Result<Arguments, String> {
    Ok(value)
}
