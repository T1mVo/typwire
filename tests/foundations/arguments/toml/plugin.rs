// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-arguments-toml")]
fn roundtrip_arguments_toml(value: Arguments) -> Result<Arguments, String> {
    Ok(value)
}
