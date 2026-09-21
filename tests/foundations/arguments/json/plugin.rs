// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-arguments-json")]
fn roundtrip_arguments_json(value: Arguments) -> Result<Arguments, String> {
    Ok(value)
}
