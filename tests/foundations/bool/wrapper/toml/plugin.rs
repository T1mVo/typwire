// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-bool-wrapper-toml")]
fn roundtrip_bool_wrapper_toml(value: typwire::Bool) -> Result<typwire::Bool, String> {
    Ok(value)
}
